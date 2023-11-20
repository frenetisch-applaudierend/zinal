#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{Expr, ItemStruct};

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

    let Some(content) = options.content else {
        todo!("Not yet implemented");
    };

    let Some(content_type) = options.content_type else {
        todo!("Not yet implemented");
    };

    let items = parser::parse(&content, &content_type)?;
    let items = items.iter().map(|item| match item {
        parser::Item::Literal(s) => quote! { write!(w, "{}", #s)?; },
        parser::Item::Expression(expr) => {
            quote! { ::stardust::Renderable::render_to(&#expr, w)?; }
        }
        parser::Item::Statement(_) => quote! { todo!(); },
    });

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
