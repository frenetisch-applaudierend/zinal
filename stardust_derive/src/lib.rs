#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;

use std::{ffi::OsStr, path::PathBuf};

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::ItemStruct;

use crate::opts::TemplateOptions;

mod opts;
mod parser;

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemStruct);

    match derive_template_inner(input) {
        Ok(stream) => stream,
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_template_inner(input: ItemStruct) -> Result<TokenStream, syn::Error> {
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

    let items = parser::parse(&content, &content_type)?;
    let items = items.into_iter().map(emit_tokens);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::stardust::Renderable for #name #ty_generics #where_clause {
            fn render_to(&self, w: &mut dyn ::std::fmt::Write) -> ::std::result::Result<(), ::std::fmt::Error> {
                #(#items)*

                Ok(())
            }
        }

        impl #impl_generics stardust::Template for #name #ty_generics #where_clause {}
    };

    // Hand the output tokens back to the compiler
    Ok(TokenStream::from(expanded))
}

fn read_content(options: TemplateOptions) -> Result<(String, String), syn::Error> {
    if options.path.is_some() {
        read_file_content(options)
    } else {
        read_inline_content(options)
    }
}

fn read_file_content(options: TemplateOptions) -> Result<(String, String), syn::Error> {
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
        std::fs::read_to_string(full_path).map_err(|e| syn::Error::new(Span::call_site(), e))?;

    Ok((content, content_type))
}

fn content_type_from_ext(ext: Option<&OsStr>) -> Result<String, syn::Error> {
    match ext.and_then(OsStr::to_str) {
        Some("html") | Some("htm") => Ok("html".to_string()),
        Some("txt") | None => Ok("plain".to_string()),
        Some(unknown) => Err(syn::Error::new(
            Span::call_site(),
            format!(
                "Unknown content type for extension '{}'. Please add explicit type attribute",
                unknown
            ),
        )),
    }
}

fn read_inline_content(options: TemplateOptions) -> Result<(String, String), syn::Error> {
    Ok((
        options.content.expect("Should have been verified"),
        options.content_type.expect("Should have been verified"),
    ))
}

fn emit_tokens(item: parser::Item<'_>) -> proc_macro2::TokenStream {
    match item {
        parser::Item::Literal(s) => quote! {
            write!(w, "{}", #s)?;
        },

        parser::Item::Expression(expr) => quote! {
            ::stardust::Renderable::render_to(&#expr, w)?;
        },

        parser::Item::BlockStatement(keyword, expr, inner) => {
            let inner = inner.into_iter().map(emit_tokens);
            quote! {
                #keyword #expr {
                    #(#inner;)*
                }
            }
        }

        parser::Item::KeywordStatement(keyword, tokens) => quote! {
            #keyword #tokens;
        },

        parser::Item::PlainStatement(tokens) => quote! {
            #tokens;
        },
    }
}
