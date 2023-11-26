use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Error;

use crate::parser::{BlockKeyword, InlineKeyword, Item};

impl Item<'_> {
    pub(crate) fn emit_all(
        items: impl IntoIterator<Item = Self>,
    ) -> Result<Vec<TokenStream>, Error> {
        items.into_iter().map(Item::emit).collect::<Result<_, _>>()
    }

    pub(crate) fn emit(self) -> Result<TokenStream, Error> {
        match self {
            Item::Literal(s) => Ok(quote! {
                write!(w, "{}", #s)?;
            }),

            Item::Expression(expr) => {
                let expr = syn::parse_str::<syn::Expr>(expr.as_ref())?;
                Ok(quote! {
                    ::stardust::Renderable::render_to(&#expr, w)?;
                })
            }

            Item::BlockStatement {
                keyword,
                expr,
                body,
            } => {
                let expr = match expr {
                    Some(expr) => Some(syn::parse_str::<syn::Expr>(expr)?),
                    None => None,
                };
                let body = Item::emit_all(body)?;

                Ok(quote! {
                    #keyword #expr {
                        #(#body)*
                    }
                })
            }

            Item::KeywordStatement { keyword, statement } => {
                let statement = match statement {
                    Some(s) => Some(syn::parse_str::<TokenStream>(s)?),
                    None => None,
                };

                Ok(quote! {
                    #keyword #statement;
                })
            }

            Item::PlainStatement(tokens) => Ok(quote! {
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

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use proc_macro2::TokenStream;
    use syn::Error;

    use crate::parser::{BlockKeyword, Item};

    #[test]
    fn literal() {
        let items = vec![Item::Literal("Hello, World!")];

        let tokens = Item::emit_all(items);

        let expected = quote! { write!(w, "{}", "Hello, World!")?; };

        assert_text(tokens, expected);
    }

    #[test]
    fn expression() {
        let items = vec![Item::Expression(Cow::from("self.name.to_upper()"))];

        let tokens = Item::emit_all(items);

        let expected = quote! { ::stardust::Renderable::render_to(&self.name.to_upper(), w)?; };

        assert_text(tokens, expected);
    }

    #[test]
    fn combination() {
        let items = vec![
            Item::Literal("Hello, "),
            Item::Expression(Cow::from("self.name.to_upper()")),
            Item::Literal("!"),
        ];

        let tokens = Item::emit_all(items);

        let expected = quote! {
            write!(w, "{}", "Hello, ")?;
            ::stardust::Renderable::render_to(&self.name.to_upper(), w)?;
            write!(w, "{}", "!")?;
        };

        assert_text(tokens, expected);
    }

    #[test]
    fn block_statement_if() {
        let items = vec![Item::BlockStatement {
            keyword: BlockKeyword::If,
            expr: Some("self.age > 18"),
            body: vec![Item::Literal("Hello, World!")],
        }];

        let tokens = Item::emit_all(items);

        let expected = quote! {
            if self.age > 18 {
                write!(w, "{}", "Hello, World!")?;
            }
        };

        assert_text(tokens, expected);
    }

    #[test]
    fn block_statement_loop() {
        let items = vec![Item::BlockStatement {
            keyword: BlockKeyword::Loop,
            expr: None,
            body: vec![Item::Literal("Hello, World!")],
        }];

        let tokens = Item::emit_all(items);

        let expected = quote! {
            loop {
                write!(w, "{}", "Hello, World!")?;
            }
        };

        assert_text(tokens, expected);
    }

    fn assert_text(tokens: Result<Vec<TokenStream>, Error>, expected: TokenStream) {
        let expected: String = expected.to_string();

        assert!(tokens.is_ok_and(|tokens| {
            let tokens = tokens.into_iter().collect::<TokenStream>().to_string();
            assert_eq!(tokens, expected);
            true
        }));
    }
}
