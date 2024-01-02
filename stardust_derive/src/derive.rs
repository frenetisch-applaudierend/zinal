use std::{ffi::OsStr, path::PathBuf};

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

    let (content, content_type) = read_content(options)?;

    let (items, content_type_ty) = parser::parse(&content, &content_type)?;
    let items = Item::emit_all(items)?;

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::stardust::Template<#content_type_ty> for #name #ty_generics #where_clause {
            fn render(&self, __stardust_context: &mut ::stardust::RenderContext<#content_type_ty>) -> ::std::result::Result<(), ::std::fmt::Error> {
                #(#items)*

                Ok(())
            }
        }
    };

    // Hand the output tokens back to the compiler
    Ok(expanded)
}

fn read_content(options: TemplateOptions) -> Result<(String, String), Error> {
    if options.path.is_some() {
        read_file_content(options)
    } else {
        read_inline_content(options)
    }
}

fn read_file_content(options: TemplateOptions) -> Result<(String, String), Error> {
    let file_name = PathBuf::from(options.path.expect("Should have been verified"));
    let content_type = if let Some(content_type) = options.content_type {
        content_type
    } else {
        content_type_from_ext(file_name.extension())?
    };

    let mut full_path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("Internal error: environmental variable `CARGO_MANIFEST_DIR` is not set."),
    );
    full_path.push("templates");
    full_path.push(file_name);

    let content =
        std::fs::read_to_string(full_path).map_err(|e| Error::new(Span::call_site(), e))?;

    Ok((content, content_type))
}

fn content_type_from_ext(ext: Option<&OsStr>) -> Result<String, Error> {
    match ext.and_then(OsStr::to_str) {
        Some("html") | Some("htm") => Ok("html".to_string()),
        Some("txt") | None => Ok("plain".to_string()),
        Some(unknown) => Err(Error::new(
            Span::call_site(),
            format!(
                "Unknown content type for extension '{}'. Please add explicit type attribute",
                unknown
            ),
        )),
    }
}

fn read_inline_content(options: TemplateOptions) -> Result<(String, String), Error> {
    Ok((
        options.content.expect("Should have been verified"),
        options.content_type.expect("Should have been verified"),
    ))
}
