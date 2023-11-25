#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::ItemStruct;

mod derive;
mod emit;
mod opts;
mod parser;

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemStruct);

    match derive::derive_template(input) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
