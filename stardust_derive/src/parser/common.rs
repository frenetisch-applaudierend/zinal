use super::Error;

pub fn parse_rust_expr(source: &str, end: &str, escape: &str) -> Result<(syn::Expr, usize), Error> {
    Ok((parse_quote! { "Hello, parse_rust_expr" }, 1))
}

pub fn parse_rust_code(
    source: &str,
    end: &str,
    escape: &str,
) -> Result<(proc_macro2::TokenStream, usize), Error> {
    Ok((quote! { println!("Hello, parse_rust_code"); }, 1))
}
