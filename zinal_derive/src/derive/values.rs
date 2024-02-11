use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{Field, FieldMutability, FieldsNamed, Generics, ItemStruct, Type, Visibility};

use crate::derive::fields::Optionality;

use super::fields::TemplateFields;

pub struct TemplateValues<'a> {
    pub ident: Ident,
    pub generics: Generics,
    pub values_fields: FieldsNamed,
    template_fields: &'a TemplateFields,
}

impl<'a> TemplateValues<'a> {
    pub fn from_template(template: &ItemStruct, fields: &'a TemplateFields) -> Self {
        let values_fields = FieldsNamed {
            brace_token: Default::default(),
            named: fields
                .all()
                .map(|f| {
                    let orig_ty = &f.ty;

                    Field {
                        attrs: Vec::new(),
                        vis: Visibility::Inherited,
                        mutability: FieldMutability::None,
                        ident: Some(f.ident.clone()),
                        colon_token: Default::default(),
                        ty: parse_quote!(::std::option::Option<#orig_ty>),
                    }
                })
                .collect(),
        };

        Self {
            ident: super::generated_ident(template, "Values"),
            generics: template.generics.clone(),
            values_fields,
            template_fields: fields,
        }
    }

    pub fn generate_decl(&self) -> TokenStream {
        let ident = &self.ident;
        let generics = &self.generics;
        let fields = &self.values_fields;

        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            struct #ident #generics #fields
        }
    }

    pub fn generate_default_impl(&self) -> TokenStream {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let fields = self
            .template_fields
            .all()
            .map(|f| {
                let ident = &f.ident;
                let value = match &f.optionality {
                    Optionality::Required => quote!(::std::option::Option::None),
                    Optionality::Optional(expr) => quote!(::std::option::Option::Some(#expr)),
                };
                quote!(#ident: #value)
            })
            .collect::<Vec<_>>();

        quote! {
            #[automatically_derived]
            impl #impl_generics ::std::default::Default for #ident #ty_generics #where_clause {
                fn default() -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        }
    }

    pub fn ty_ref(&self) -> Type {
        let ident = &self.ident;
        let (_, ty_generics, _) = self.generics.split_for_impl();

        parse_quote!(#ident #ty_generics)
    }
}

impl ToTokens for TemplateValues<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generate_decl().to_tokens(tokens);
        self.generate_default_impl().to_tokens(tokens);
    }
}
