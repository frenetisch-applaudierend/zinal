use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Error;

use crate::parser::{Item, Keyword, TemplateArgument, TemplateArgumentValue};

trait Emit {
    fn emit(self) -> Result<TokenStream, Error>;
}

impl Item<'_> {
    pub(crate) fn emit_all(
        items: impl IntoIterator<Item = Self>,
    ) -> Result<Vec<TokenStream>, Error> {
        items.into_iter().map(Emit::emit).collect::<Result<_, _>>()
    }
}

impl Emit for Item<'_> {
    fn emit(self) -> Result<TokenStream, Error> {
        match self {
            Item::Literal(s) => Ok(quote! {
                __stardust_context.render_literal(#s)?;
            }),

            Item::Expression(expr) => {
                let expr = syn::parse_str::<syn::Expr>(expr.as_ref())?;
                Ok(quote! {
                    __stardust_context.render_expression(&#expr)?;
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

            Item::ChildTemplate {
                name,
                arguments,
                children,
            } => {
                let ty = syn::parse_str::<syn::TypePath>(name.as_ref())?;
                let mut arguments = arguments
                    .into_iter()
                    .map(Emit::emit)
                    .collect::<Result<Vec<_>, _>>()?;

                if !children.is_empty() {
                    let children = Item::emit_all(children)?;
                    let tokens = quote! {
                        children: ::stardust::Children::new(|__stardust_context| {
                            #(#children)*

                            Ok(())
                        })
                    };
                    arguments.push(tokens);
                }

                let template = quote! {
                    #ty { #(#arguments),* }
                };

                Ok(quote! {
                    __stardust_context.render_template(#template)?;
                })
            }
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

            Keyword::End => unreachable!(),
        };
        keyword.to_tokens(tokens);
    }
}

impl Emit for TemplateArgument<'_> {
    fn emit(self) -> Result<TokenStream, Error> {
        let name = syn::parse_str::<syn::Ident>(self.name.as_ref())?;
        let value = self.value.emit()?;

        Ok(quote! {
            #name: #value
        })
    }
}

impl Emit for TemplateArgumentValue<'_> {
    fn emit(self) -> Result<TokenStream, Error> {
        Ok(match self {
            TemplateArgumentValue::Literal(v) => quote!(#v.into()),
            TemplateArgumentValue::Expression(expr) => {
                syn::parse_str::<TokenStream>(expr.as_ref())?
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use proc_macro2::TokenStream;
    use syn::Error;

    use crate::parser::{Item, Keyword, TemplateArgument, TemplateArgumentValue};

    #[test]
    fn literal() {
        let items = vec![Item::Literal(Cow::from("Hello, World!"))];

        let tokens = Item::emit_all(items);

        let expected = quote! {
            __stardust_context.render_literal("Hello, World!")?;
        };

        assert_text(tokens, expected);
    }

    #[test]
    fn expression() {
        let items = vec![Item::Expression(Cow::from("self.name.to_upper()"))];

        let tokens = Item::emit_all(items);

        let expected = quote! {
            __stardust_context.render_expression(&self.name.to_upper())?;
        };

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
            __stardust_context.render_literal("Hello, ")?;
            __stardust_context.render_expression(&self.name.to_upper())?;
            __stardust_context.render_literal("!")?;
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
                __stardust_context.render_literal("Hello, World!")?;
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
                __stardust_context.render_literal("Hello, World!")?;
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
                __stardust_context.render_literal("Hello, World!")?;
            }
        };

        assert_text(tokens, expected);
    }

    #[test]
    fn child_template() {
        let items = vec![Item::ChildTemplate {
            name: Cow::from("::module::Type"),
            arguments: vec![
                TemplateArgument {
                    name: Cow::from("expr"),
                    value: TemplateArgumentValue::Expression(Cow::from("self.name")),
                },
                TemplateArgument {
                    name: Cow::from("lit"),
                    value: TemplateArgumentValue::Literal(Cow::from("Literal")),
                },
            ],
            children: vec![],
        }];

        let tokens = Item::emit_all(items);

        let expected = quote! {
            __stardust_context.render_template(::module::Type {
                expr: self.name,
                lit: "Literal".into()
            })?;
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
