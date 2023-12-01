use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Error;

use crate::parser::{Item, Keyword};

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

            Item::KeywordStatement {
                keyword,
                statement,
                body,
            } => {
                let statement = match statement {
                    Some(s) => Some(syn::parse_str::<TokenStream>(s.as_ref())?),
                    None => None,
                };
                let body = Item::emit_all(body)?;

                Ok(quote! {
                    #keyword #statement {
                        #(#body)*
                    }
                })
            }

            Item::PlainStatement(tokens) => Ok(quote! {
                #tokens;
            }),
        }
    }
}

impl ToTokens for Keyword {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let keyword = match self {
            Keyword::If => quote!(if),
            Keyword::Else => quote!(else),
            Keyword::ElseIf => quote!(else if),
            Keyword::For => quote!(for),
            Keyword::While => quote!(while),
            Keyword::Loop => quote!(loop),
            Keyword::Break => quote!(break),
            Keyword::Continue => quote!(continue),
            Keyword::Let => quote!(let),
        };
        keyword.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use proc_macro2::TokenStream;
    use syn::Error;

    use crate::parser::{Item, Keyword};

    #[test]
    fn literal() {
        let items = vec![Item::Literal(Cow::from("Hello, World!"))];

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
            Item::Literal(Cow::from("Hello, ")),
            Item::Expression(Cow::from("self.name.to_upper()")),
            Item::Literal(Cow::from("!")),
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
    fn keyword_statement_if() {
        let items = vec![Item::KeywordStatement {
            keyword: Keyword::If,
            statement: Some(Cow::from("self.age > 18")),
            body: vec![Item::Literal(Cow::from("Hello, World!"))],
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
    fn keyword_statement_loop() {
        let items = vec![Item::KeywordStatement {
            keyword: Keyword::Loop,
            statement: None,
            body: vec![Item::Literal(Cow::from("Hello, World!"))],
        }];

        let tokens = Item::emit_all(items);

        let expected = quote! {
            loop {
                write!(w, "{}", "Hello, World!")?;
            }
        };

        assert_text(tokens, expected);
    }

    #[test]
    fn keyword_statement_for() {
        let items = vec![Item::KeywordStatement {
            keyword: Keyword::For,
            statement: Some(Cow::from("name in self.names")),
            body: vec![Item::Literal(Cow::from("Hello, World!"))],
        }];

        let tokens = Item::emit_all(items);

        let expected = quote! {
            for name in self.names {
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
