use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};

use syn::{Error, ItemStruct};

use crate::{
    opts::TemplateOptions,
    parser::{self, Item},
};

pub(crate) fn derive_template(input: ItemStruct) -> Result<TokenStream, Error> {
    let mut options = TemplateOptions::default();

    for attr in input
        .attrs
        .into_iter()
        .filter(|a| a.path().is_ident("template"))
    {
        options.merge_attr(&attr)?;
    }

    options.validate()?;

    let content = read_content(options)?;

    let items = parser::parse(&content)?;
    let items = Item::emit_all(items)?;

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut expanded = TokenStream::new();

    expanded.extend(quote! {
        impl #impl_generics ::stardust::Template for #name #ty_generics #where_clause {
            fn render(&self, __stardust_context: &mut ::stardust::RenderContext) -> ::std::result::Result<(), ::std::fmt::Error> {
                #(#items)*

                Ok(())
            }
        }
    });

    #[cfg(feature = "axum")]
    expanded.extend(quote! {
        impl #impl_generics ::core::convert::Into<::axum::body::Body> for #name #ty_generics #where_clause {
            fn into(self) -> ::axum::body::Body {
                self.render_to_string().expect("Could not render template to string").into()
            }
        }
    });

    // Hand the output tokens back to the compiler
    Ok(expanded)
}

fn read_content(options: TemplateOptions) -> Result<String, Error> {
    if options.path.is_some() {
        read_file_content(options)
    } else {
        read_inline_content(options)
    }
}

fn read_file_content(options: TemplateOptions) -> Result<String, Error> {
    let file_name = PathBuf::from(options.path.expect("Should have been verified"));

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

fn read_inline_content(options: TemplateOptions) -> Result<String, Error> {
    Ok(options.content.expect("Should have been verified"))
}
