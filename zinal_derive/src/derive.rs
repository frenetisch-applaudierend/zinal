use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};

use syn::{Error, Ident, ItemStruct};

use crate::{
    opts::TemplateOptions,
    parser::{self, Item},
};

mod builder;
mod fields;
mod properties;
mod values;

use builder::*;
use fields::*;
use properties::*;
use values::*;

pub(crate) fn derive(template: ItemStruct) -> Result<TokenStream, Error> {
    let options = TemplateOptions::from_struct(&template)?;

    let fields = TemplateFields::from_template(&template)?;
    let properties = TemplateProperties::from_template(&template, &fields);
    let values = TemplateValues::from_template(&template, &fields);
    let builder = TemplateBuilder::from_template(&template, &fields, &values, &properties);

    let template_impl = derive_template_impl(&template, &fields, &options, &builder)?;

    Ok(quote! {
        #template_impl
        #values
        #properties
        #builder
    })
}

fn derive_template_impl(
    template: &ItemStruct,
    fields: &TemplateFields,
    options: &TemplateOptions,
    builder: &TemplateBuilder<'_>,
) -> Result<TokenStream, Error> {
    let content = read_content(options)?;

    let items = parser::parse(&content)?;
    let items = Item::emit_all(items)?;

    let providers = derive_context_providers(fields);

    let ident = &template.ident;
    let (impl_generics, ty_generics, where_clause) = template.generics.split_for_impl();

    let builder_ty = &builder.ident;
    let builder_args = builder.generic_args(parse_quote!(()));

    let mut expanded = TokenStream::new();

    expanded.extend(quote! {

        #[automatically_derived]
        impl #impl_generics ::zinal::Template for #ident #ty_generics #where_clause {
            type Builder = #builder_ty #builder_args;

            fn render(
                self,
                __zinal_writer: &mut dyn ::std::fmt::Write,
                __zinal_escaper: &dyn ::zinal::Escaper,
                __zinal_context: &::zinal::Context,
                __zinal_children: impl ::zinal::Children,
            ) -> ::std::result::Result<(), ::std::fmt::Error> {
                let mut __zinal_provided_context = ::zinal::Context::new();
                #(#providers)*
                let __zinal_context = &__zinal_context.extend(__zinal_provided_context);

                #(#items)*

                Ok(())
            }

            fn builder() -> Self::Builder {
                #builder_ty::new()
            }
        }
    });

    #[cfg(feature = "axum")]
    expanded.extend(quote! {

        #[automatically_derived]
        impl #impl_generics ::core::convert::Into<::axum::body::Body> for #ident #ty_generics #where_clause {
            fn into(self) -> ::axum::body::Body {
                self.render_to_string().expect("Could not render template to string").into()
            }
        }
    });

    // Hand the output tokens back to the compiler
    Ok(expanded)
}

fn derive_context_providers(fields: &TemplateFields) -> Vec<TokenStream> {
    fields
        .args()
        .filter(|f| f.provides_context)
        .map(|f| {
            let ident = &f.ident;
            quote!(
                __zinal_provided_context.provide_param(self.#ident.clone());
            )
        })
        .collect()
}

pub(crate) fn generated_ident(template: &ItemStruct, name: &str) -> Ident {
    Ident::new(
        &format!("__zinal_generated_{}_{}", template.ident, name),
        template.ident.span(),
    )
}

fn read_content(options: &TemplateOptions) -> Result<String, Error> {
    if options.path.is_some() {
        read_file_content(options)
    } else {
        read_inline_content(options)
    }
}

fn read_file_content(options: &TemplateOptions) -> Result<String, Error> {
    let file_name = PathBuf::from(options.path.as_ref().expect("Should have been verified"));

    let mut full_path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("Internal error: environmental variable `CARGO_MANIFEST_DIR` is not set."),
    );
    full_path.push("templates");
    full_path.push(file_name);

    let content =
        std::fs::read_to_string(full_path).map_err(|e| Error::new(Span::call_site(), e))?;

    Ok(content)
}

fn read_inline_content(options: &TemplateOptions) -> Result<String, Error> {
    Ok(options.content.clone().expect("Should have been verified"))
}
