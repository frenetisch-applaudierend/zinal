use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{
    punctuated::Punctuated, token::Brace, Field, FieldMutability, FieldsNamed, Generics,
    ItemStruct, Type, Visibility,
};

pub struct TemplateValues {
    pub ident: Ident,
    pub generics: Generics,
    pub fields: FieldsNamed,
}

impl TemplateValues {
    pub fn from_template(template: &ItemStruct, fields: Option<&FieldsNamed>) -> Self {
        let fields = match fields {
            Some(fs) => FieldsNamed {
                brace_token: Brace::default(),
                named: fs
                    .named
                    .iter()
                    .map(|f| {
                        let orig_ty = &f.ty;

                        Field {
                            attrs: Vec::new(),
                            vis: Visibility::Inherited,
                            mutability: FieldMutability::None,
                            ident: f.ident.clone(),
                            colon_token: Default::default(),
                            ty: parse_quote!(::std::option::Option<#orig_ty>),
                        }
                    })
                    .collect(),
            },
            None => FieldsNamed {
                brace_token: Brace::default(),
                named: Punctuated::new(),
            },
        };

        Self {
            ident: super::generated_ident(template, "Values"),
            generics: template.generics.clone(),
            fields,
        }
    }

    pub fn generate_decl(&self) -> TokenStream {
        let ident = &self.ident;
        let generics = &self.generics;
        let fields = &self.fields;

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
            .fields
            .named
            .iter()
            .map(|f| {
                let ident = &f.ident;
                quote!(#ident: ::std::option::Option::None)
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

    pub fn initial_token_ty(&self) -> Type {
        // TODO: Add logic once we support default params
        parse_quote!(())
    }

    pub fn ty_ref(&self) -> Type {
        let ident = &self.ident;
        let (_, ty_generics, _) = self.generics.split_for_impl();

        parse_quote!(#ident #ty_generics)
    }
}

impl ToTokens for TemplateValues {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generate_decl().to_tokens(tokens);
        self.generate_default_impl().to_tokens(tokens);
    }
}
