use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{ItemStruct, Type};

use super::fields::{Source, TemplateFields};

pub struct TemplateProperties {
    pub mod_ident: Ident,
    pub properties: Vec<Ident>,
}

impl TemplateProperties {
    pub fn from_template(template: &ItemStruct, fields: &TemplateFields) -> Self {
        Self {
            mod_ident: super::generated_ident(template, "Properties"),
            properties: fields
                .iter()
                .filter(|f| matches!(f.source, Source::Argument))
                .map(|f| f.ident.clone())
                .collect(),
        }
    }

    pub fn prop_ty(&self, prop: &Ident) -> Type {
        let mod_ident = &self.mod_ident;
        parse_quote!(#mod_ident::#prop)
    }

    pub fn generate_decl(&self) -> TokenStream {
        let mod_ident = &self.mod_ident;
        let property_tys = self
            .properties
            .iter()
            .map(|ident| {
                quote! {
                    #[allow(non_camel_case_types)]
                    pub struct #ident;
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            pub mod #mod_ident {
                #(#property_tys)*
            }
        }
    }
}

impl ToTokens for TemplateProperties {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generate_decl().to_tokens(tokens);
    }
}
