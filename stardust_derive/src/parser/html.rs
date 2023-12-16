use std::borrow::Cow;

use parser_common::{literal, select, take_until, whitespace, Boxed, Input, Parser};
use proc_macro2::Span;

use crate::parser::Keyword;

use super::{Item, TemplateParser};

pub struct HtmlParser;

struct KeywordTag<'src> {
    keyword: Keyword,
    statement: Option<Cow<'src, str>>,
}

impl TemplateParser for HtmlParser {
    fn parse<'src>(&mut self, mut input: Input<'src>) -> Result<Vec<Item<'src>>, syn::Error> {
        Ok(template_item()
            .repeated()
            .parse(&mut input)
            .map_err(|err| syn::Error::new(Span::call_site(), err.to_string()))?
            .expect("Expected a Vec<Item>"))
    }
}

fn template_item<'src>() -> Boxed<'src, Item<'src>> {
    select((
        escape(),
        expression(),
        statement(),
        // parse_child_template,
        // parse_literal,
    ))
    .boxed()
}

fn escape<'src>() -> impl Parser<'src, Output = Item<'src>> {
    select((
        literal("{{").map(|_| Cow::from("{")),
        literal("<##").map(|_| Cow::from("<#")),
    ))
    .map(Item::Literal)
}

fn expression<'src>() -> impl Parser<'src, Output = Item<'src>> {
    let start = || literal("{").then(whitespace());
    let end = || whitespace().then(literal("}"));

    start()
        .ignore_then(take_until(end()).escape("}}", "}").map(Item::Expression))
        .then_expect_ignore(end())
}

fn statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
    return select((keyword_statement(), plain_statement()));

    fn keyword_statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
        return select((simple_keyword_statement(), block_keyword_statement()));

        fn simple_keyword_statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
            let simple_keywords = || {
                select((
                    literal("break").to(Keyword::Break),
                    literal("continue").to(Keyword::Continue),
                    literal("let").to(Keyword::Let),
                ))
            };

            keyword_tag(simple_keywords).map(|tag| Item::KeywordStatement {
                keyword: tag.keyword,
                statement: tag.statement,
                body: Vec::new(),
            })
        }

        fn block_keyword_statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
            let block_keywords = || {
                select((
                    literal("if").to(Keyword::If),
                    literal("else").to(Keyword::Else),
                    literal("else")
                        .then(whitespace())
                        .then(literal("if"))
                        .to(Keyword::ElseIf),
                    literal("for").to(Keyword::For),
                    literal("while").to(Keyword::While),
                    literal("loop").to(Keyword::Loop),
                ))
            };

            let block_end_keywords = || {
                select((
                    literal("end").to(Keyword::End),
                    literal("else").to(Keyword::Else),
                    literal("else")
                        .then(whitespace())
                        .then(literal("if"))
                        .to(Keyword::ElseIf),
                ))
            };

            let block_end = || keyword_tag(block_end_keywords);
            let end_tag = || keyword_tag(|| literal("end").to(Keyword::End));

            keyword_tag(block_keywords)
                .then(template_item().repeated_until(block_end()))
                .then_ignore(end_tag().optional())
                .map(|(tag, body)| Item::KeywordStatement {
                    keyword: tag.keyword,
                    statement: tag.statement,
                    body,
                })
        }

        fn keyword_tag<'src, F, K>(keyword: F) -> impl Parser<'src, Output = KeywordTag<'src>>
        where
            F: Fn() -> K,
            K: Parser<'src, Output = Keyword>,
        {
            return select((shorthand_tag(keyword()), longform_tag(keyword())));

            fn shorthand_tag<'src>(
                keyword: impl Parser<'src, Output = Keyword>,
            ) -> impl Parser<'src, Output = KeywordTag<'src>> {
                literal("<#")
                    .ignore_then(whitespace())
                    .ignore_then(keyword)
                    .then_ignore(whitespace())
                    .then_ignore(literal(">"))
                    .map(|k| KeywordTag {
                        keyword: k,
                        statement: None,
                    })
            }

            fn longform_tag<'src>(
                keyword: impl Parser<'src, Output = Keyword>,
            ) -> impl Parser<'src, Output = KeywordTag<'src>> {
                let end = || whitespace().then(literal("#>"));

                literal("<#")
                    .ignore_then(whitespace())
                    .ignore_then(keyword)
                    .then_ignore(whitespace().not_empty())
                    .then(take_until(end()).escape("##>", "#>").optional())
                    .then_ignore(end())
                    .map(|(k, s)| KeywordTag {
                        keyword: k,
                        statement: s,
                    })
            }
        }
    }

    fn plain_statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
        let start = || literal("<#").then(whitespace());
        let end = || whitespace().then(literal("#>"));

        start()
            .ignore_then(
                take_until(end())
                    .escape("##>", "#>")
                    .map(Item::PlainStatement),
            )
            .then_expect_ignore(end())
    }
}

#[cfg(test)]
mod tests;
