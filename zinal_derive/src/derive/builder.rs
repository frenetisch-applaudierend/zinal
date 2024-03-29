use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::{
    punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments, GenericArgument,
    GenericParam, Generics, ItemStruct, Type, WhereClause,
};

use super::{
    fields::{TemplateField, TemplateFields},
    properties::TemplateProperties,
    values::TemplateValues,
};

pub struct TemplateBuilder<'a> {
    pub ident: Ident,
    generics: Generics,
    args_template: Punctuated<GenericArgument, Comma>,
    template_ident: &'a Ident,
    template_generics: &'a Generics,
    template_fields: &'a TemplateFields,
    values: &'a TemplateValues<'a>,
    properties: &'a TemplateProperties,
}

impl<'a> TemplateBuilder<'a> {
    pub fn from_template(
        template: &'a ItemStruct,
        fields: &'a TemplateFields,
        values: &'a TemplateValues,
        properties: &'a TemplateProperties,
    ) -> Self {
        let mut generics = template.generics.clone();
        generics
            .params
            .push(GenericParam::Type(parse_quote!(__zinal_Token)));

        let mut args_template = Punctuated::new();

        for param in template.generics.params.iter() {
            match param {
                GenericParam::Lifetime(l) => {
                    args_template.push(GenericArgument::Lifetime(l.lifetime.clone()));
                }
                GenericParam::Type(t) => {
                    let ident = &t.ident;
                    args_template.push(GenericArgument::Type(parse_quote!(#ident)));
                }
                GenericParam::Const(c) => {
                    let ident = &c.ident;
                    args_template.push(GenericArgument::Const(parse_quote!(#ident)));
                }
            }
        }

        Self {
            ident: super::generated_ident(template, "Builder"),
            generics,
            args_template,
            template_ident: &template.ident,
            template_generics: &template.generics,
            template_fields: fields,
            values,
            properties,
        }
    }

    pub fn generic_args(&self, token_ty: Type) -> AngleBracketedGenericArguments {
        let mut args = self.args_template.clone();
        args.push(GenericArgument::Type(token_ty));

        AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args,
            gt_token: Default::default(),
        }
    }

    pub fn generate_decl(&self) -> TokenStream {
        let ident = &self.ident;
        let generics = &self.generics;
        let values = &self.values.ty_ref();

        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            pub struct #ident #generics (::zinal::builder::TemplateBuilder<#values, __zinal_Token>);
        }
    }

    pub fn generate_impl(&self) -> TokenStream {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let setters = self
            .template_fields
            .args()
            .map(|f| self.generate_setter(f))
            .collect::<Vec<_>>();

        let build_method = self.generate_build_method();

        quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                pub fn new() -> Self {
                    Self(::std::default::Default::default())
                }

                #(#setters)*

                #build_method
            }
        }
    }

    fn generate_setter(&self, field: &TemplateField) -> TokenStream {
        let builder_ident = &self.ident;
        let field_ident = &field.ident;
        let field_ty = &field.ty;
        let prop = self.properties.prop_ty(field_ident);

        let builder_args =
            self.generic_args(parse_quote!(::zinal::builder::WithProperty<#prop, __zinal_Token>));

        quote! {
            pub fn #field_ident(self, value: #field_ty) -> #builder_ident #builder_args {
                #builder_ident(self.0.set::<#prop>(|values| { values.#field_ident = ::std::option::Option::Some(value); }))
            }
        }
    }

    fn generate_build_method(&self) -> TokenStream {
        let template_ident = &self.template_ident;
        let (_, template_generics, _) = self.template_generics.split_for_impl();

        let mut args = Punctuated::new();
        let mut predicates = Punctuated::new();

        args.push(parse_quote!('__zinal_ctx));

        for lifetime in self.generics.lifetimes() {
            predicates.push(parse_quote!('__zinal_ctx: #lifetime));
        }

        for field in self.template_fields.args() {
            if field.optionality.is_optional() {
                continue;
            }

            let prop_ident = &field.ident;
            let prop_tail_name = prop_ident.to_string();
            let mut prop_tail_name = prop_tail_name.as_str();
            if prop_tail_name.starts_with("r#") {
                prop_tail_name = &prop_tail_name[2..];
            }
            let tail_ident = Ident::new(&format!("Tail_{}", prop_tail_name), Span::mixed_site());
            let prop_ty = self.properties.prop_ty(prop_ident);

            args.push(parse_quote!(#tail_ident));
            predicates.push(
                parse_quote!(__zinal_Token: ::zinal::builder::HasProperty<#prop_ty, #tail_ident>),
            );
        }

        let build_params = AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args,
            gt_token: Default::default(),
        };

        let where_clause = if !predicates.is_empty() {
            Some(WhereClause {
                where_token: <Token![where]>::default(),
                predicates,
            })
        } else {
            None
        };

        let mut field_initializers: Vec<TokenStream> = Vec::new();

        for field in self.template_fields.args() {
            let field_ident = &field.ident;
            field_initializers.push(quote!(
                #field_ident: self.0.values.#field_ident.expect("value must be set")
            ));
        }

        for field in self.template_fields.ctx() {
            let field_ident = &field.ident;
            let field_name = field_ident.to_string();

            field_initializers.push(quote!(
                #field_ident: context.get_param().expect(&format!("Missing context parameter {}", #field_name))
            ));
        }

        quote! {
            pub fn build #build_params (self, context: &'__zinal_ctx ::zinal::Context) -> #template_ident #template_generics #where_clause {
                #template_ident {
                    #(#field_initializers),*
                }
            }
        }
    }
}

impl ToTokens for TemplateBuilder<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generate_decl().to_tokens(tokens);
        self.generate_impl().to_tokens(tokens);
    }
}
