use std::{ffi::OsStr, path::PathBuf};

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Error, ItemStruct};

use crate::{
    opts::TemplateOptions,
    parser::{self, BlockKeyword, InlineKeyword, Item},
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

    let items = parser::parse(&content, &content_type)?;
    let items = Item::emit_all(items)?;

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

impl Item<'_> {
    fn emit_all(items: impl IntoIterator<Item = Self>) -> Result<Vec<TokenStream>, Error> {
        items.into_iter().map(Item::emit).collect::<Result<_, _>>()
    }

    fn emit(self) -> Result<TokenStream, Error> {
        match self {
            parser::Item::Literal(s) => Ok(quote! {
                write!(w, "{}", #s)?;
            }),

            parser::Item::Expression(expr) => {
                let expr = syn::parse_str::<syn::Expr>(expr)?;
                Ok(quote! {
                    ::stardust::Renderable::render_to(&#expr, w)?;
                })
            }

            parser::Item::BlockStatement {
                keyword,
                expr,
                body,
            } => {
                let expr = syn::parse_str::<syn::Expr>(expr)?;
                let body = Item::emit_all(body)?;

                Ok(quote! {
                    #keyword #expr {
                        #(#body;)*
                    }
                })
            }

            parser::Item::KeywordStatement { keyword, statement } => {
                let statement = match statement {
                    Some(s) => Some(syn::parse_str::<TokenStream>(s)?),
                    None => None,
                };

                Ok(quote! {
                    #keyword #statement;
                })
            }

            parser::Item::PlainStatement(tokens) => Ok(quote! {
                #tokens;
            }),
        }
    }
}

impl ToTokens for BlockKeyword {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let keyword = match self {
            BlockKeyword::If => quote!(if),
            BlockKeyword::Else => quote!(else),
            BlockKeyword::For => quote!(for),
            BlockKeyword::While => quote!(while),
            BlockKeyword::Loop => quote!(loop),
        };
        keyword.to_tokens(tokens);
    }
}

impl ToTokens for InlineKeyword {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let keyword = match self {
            InlineKeyword::Break => quote!(break),
            InlineKeyword::Continue => quote!(continue),
            InlineKeyword::Let => quote!(let),
        };
        keyword.to_tokens(tokens);
    }
}
