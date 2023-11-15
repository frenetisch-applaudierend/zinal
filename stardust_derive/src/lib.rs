#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    for attr in input.attrs {
        println!("Found attr {:?}", attr.path().is_ident("template"));
    }

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #impl_generics ::stardust::Renderable for #name #ty_generics #where_clause {
            fn render_to(&self, w: &mut dyn ::std::fmt::Write) -> ::std::result::Result<(), std::fmt::Error> {
                todo!()
            }
        }

        impl #impl_generics stardust::Template for #name #ty_generics #where_clause {}
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
