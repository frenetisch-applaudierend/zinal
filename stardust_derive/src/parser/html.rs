use std::borrow::Cow;

use parser_common::{literal, select, take_until, todo, whitespace, Input, ParseResult, Parser};
use proc_macro2::Span;

use crate::parser::Keyword;

use super::{Item, TemplateParser};

pub struct HtmlParser;

impl TemplateParser for HtmlParser {
    fn parse<'src>(&mut self, mut input: Input<'src>) -> Result<Vec<Item<'src>>, syn::Error> {
        Ok(template_item()
            .repeated()
            .parse(&mut input)
            .map_err(|err| syn::Error::new(Span::call_site(), err.to_string()))?
            .expect("Expected a Vec<Item>"))
    }
}

fn template_item<'src>() -> impl Parser<'src, Output = Item<'src>> {
    select((
        escape(),
        expression(),
        statement(),
        // parse_child_template,
        // parse_literal,
    ))
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
            let simple_keywords = select((
                literal("break").to(Keyword::Break),
                literal("continue").to(Keyword::Continue),
                literal("let").to(Keyword::Let),
            ));
            keyword_tag(simple_keywords)
        }

        fn block_keyword_statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
            todo()
        }

        fn keyword_tag<'src>(
            keyword: impl Parser<'src, Output = Keyword>,
        ) -> impl Parser<'src, Output = Item<'src>> {
            todo()
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

// fn parse_keyword_statement<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
//     let Some(start) = keyword_statement_tag(keyword()).parse(input)? else {
//         return Ok(None);
//     };

//     let body = if start.keyword.requires_body() {
//         body()
//             .then_ignore(keyword_statement_tag(literal("end")).optional())
//             .parse(input)?
//             .ok_or(Error::new("Expected body after block statement"))?
//     } else {
//         vec![]
//     };

//     return Ok(Some(Item::KeywordStatement {
//         keyword: start.keyword,
//         statement: start.statement,
//         body,
//     }));

//     #[derive(Debug, Clone)]
//     struct KeywordStatementTag<'src, T> {
//         keyword: T,
//         statement: Option<Cow<'src, str>>,
//     }

//     fn keyword_statement_tag<'src, T: Clone>(
//         keyword: impl Parser<'src, Output = T> + Clone,
//     ) -> impl Parser<'src, Output = KeywordStatementTag<'src, T>> + Clone
//     where
//         T: Clone,
//     {
//         let statement = take_until(literal("#>")).escape("##>", "#>").map(Some);

//         let longform = literal("<#")
//             .ignore_then(whitespace().optional())
//             .ignore_then(keyword.clone())
//             .then_ignore(whitespace())
//             .then(statement)
//             .then_ignore(literal("#>"));

//         let shortform = literal("<#")
//             .ignore_then(whitespace().optional())
//             .ignore_then(keyword)
//             .then(insert(None))
//             .then_ignore(whitespace().optional())
//             .then_ignore(select((literal(">"), literal("#>"))));

//         select((longform, shortform))
//             .map(|(keyword, statement)| KeywordStatementTag { keyword, statement })
//     }

//     fn block_statement_end<'src>() -> impl Parser<'src, Output = ()> + Clone {
//         let end_keyword = select((
//             literal("end").map(|_| ()),
//             keyword()
//                 .filter(|k| *k == Keyword::Else || *k == Keyword::ElseIf)
//                 .map(|_| ()),
//         ));

//         keyword_statement_tag(end_keyword).map(|_| ())
//     }

//     fn keyword<'src>() -> impl Parser<'src, Output = Keyword> + Clone {
//         select((
//             literal("if").map(|_| Keyword::If),
//             literal("else")
//                 .then(whitespace())
//                 .then(literal("if"))
//                 .map(|_| Keyword::ElseIf),
//             literal("else").map(|_| Keyword::Else),
//             literal("for").map(|_| Keyword::For),
//             literal("while").map(|_| Keyword::While),
//             literal("loop").map(|_| Keyword::Loop),
//             literal("break").map(|_| Keyword::Break),
//             literal("continue").map(|_| Keyword::Continue),
//             literal("let").map(|_| Keyword::Let),
//         ))
//     }

//     fn body<'src>() -> impl Parser<'src, Output = Vec<Item<'src>>> {
//         |input: &mut Input<'src>| {
//             let Some(body) = collect_until(parse_item, block_statement_end()).parse(input)? else {
//                 return Err(Error::unexpected_eof());
//             };

//             Ok(Some(body))
//         }
//     }
// }

// fn parse_plain_statement<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
//     literal("<#")
//         .ignore_then(whitespace().optional())
//         .ignore_then(take_until("#>", "##>"))
//         .then_ignore(literal("#>"))
//         .map(Item::PlainStatement)
//         .parse(input)
// }

// fn parse_child_template<'src>(_input: &mut Input<'src>) -> ParseResult<Item<'src>> {
//     return Ok(None);

//     // fn name<'src>() -> impl Parser<'src, Output = Cow<'src, str>> {
//     //     let separator = literal("::");
//     //     let
//     //     todo!()
//     // }
// }

// fn parse_literal<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
//     // consume possible leading < or {
//     let Some(lead) = input.consume_count(1) else {
//         return Err(Error::unexpected_eof());
//     };
//     let rest = input
//         .consume_until_any("<{")
//         .unwrap_or_else(|| input.consume_all());
//     let combined = input.combine(&[lead, rest]);

//     Ok(Some(Item::Literal(combined.into_cow())))
// }

#[cfg(test)]
mod tests;
