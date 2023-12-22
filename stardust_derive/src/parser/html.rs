use std::borrow::Cow;

use proc_macro2::Span;

use crate::parser::Keyword;

use super::{input::Input, Item, TemplateParser};

pub struct HtmlParser;

type ParseResult<'src, T = Item<'src>> = Result<Option<T>, syn::Error>;

struct KeywordTag<'src> {
    keyword: Keyword,
    statement: Option<Cow<'src, str>>,
}

impl TemplateParser for HtmlParser {
    fn parse<'src>(&mut self, mut input: Input<'src>) -> Result<Vec<Item<'src>>, syn::Error> {
        let mut items = Vec::new();

        while !input.is_at_end() {
            items.push(parse_template_item(&mut input)?.expect("Should never be None"));
        }

        return Ok(items);
    }
}

fn parse_template_item<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    select3(input, (parse_escape, parse_expression, parse_statement))
}

fn parse_escape<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    if input.consume_lit("{{").is_some() {
        return Ok(Some(Item::Literal(Cow::from("{"))));
    }

    if input.consume_lit("<##").is_some() {
        return Ok(Some(Item::Literal(Cow::from("<#"))));
    }

    Ok(None)
}

fn parse_expression<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    if !input.consume_lit("{").is_some() {
        return Ok(None);
    }

    let mut content = Cow::<'src, str>::Borrowed("");

    loop {
        let Some(part) = input.consume_until("}") else {
            return Err(syn::Error::new(
                Span::call_site(),
                "Unterminated expression",
            ));
        };

        append(&mut content, part.into_str());

        if input.consume_lit("}}").is_some() {
            append(&mut content, "}");
        } else if input.consume_lit("}").is_some() {
            break;
        }
    }

    Ok(Some(Item::Expression(trim(content))))
}

fn parse_statement<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    return select2(input, (parse_keyword_statement, parse_plain_statement));

    fn parse_keyword_statement<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
        Ok(None)
    }

    fn parse_plain_statement<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
        if !input.consume_lit("<#").is_some() {
            return Ok(None);
        }

        let mut content = Cow::<'src, str>::Borrowed("");

        loop {
            println!("Currently at offset {:?}", input.position());

            let Some(part) = input.consume_until("#") else {
                return Err(syn::Error::new(Span::call_site(), "Unterminated statement"));
            };

            append(&mut content, part.into_str());

            if input.consume_lit("##>").is_some() {
                append(&mut content, "#>");
            } else if input.consume_lit("#>").is_some() {
                break;
            } else {
                input.consume_lit("#").expect("Implied by consume_until");
                append(&mut content, "#");
            }
        }

        Ok(Some(Item::PlainStatement(trim(content))))
    }

    // return select((keyword_statement(), plain_statement()));

    // fn keyword_statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
    //     return select((simple_keyword_statement(), block_keyword_statement()));

    //     fn simple_keyword_statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
    //         let simple_keywords = || {
    //             select((
    //                 literal("break").to(Keyword::Break),
    //                 literal("continue").to(Keyword::Continue),
    //                 literal("let").to(Keyword::Let),
    //             ))
    //         };

    //         keyword_tag(simple_keywords).map(|tag| Item::KeywordStatement {
    //             keyword: tag.keyword,
    //             statement: tag.statement,
    //             body: Vec::new(),
    //         })
    //     }

    //     fn block_keyword_statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
    //         let block_keywords = || {
    //             select((
    //                 literal("if").to(Keyword::If),
    //                 literal("else").to(Keyword::Else),
    //                 literal("else")
    //                     .then(whitespace())
    //                     .then(literal("if"))
    //                     .to(Keyword::ElseIf),
    //                 literal("for").to(Keyword::For),
    //                 literal("while").to(Keyword::While),
    //                 literal("loop").to(Keyword::Loop),
    //             ))
    //         };

    //         let block_end_keywords = || {
    //             select((
    //                 literal("end").to(Keyword::End),
    //                 literal("else").to(Keyword::Else),
    //                 literal("else")
    //                     .then(whitespace())
    //                     .then(literal("if"))
    //                     .to(Keyword::ElseIf),
    //             ))
    //         };

    //         let block_end = || keyword_tag(block_end_keywords);
    //         let end_tag = || keyword_tag(|| literal("end").to(Keyword::End));

    //         keyword_tag(block_keywords)
    //             .then(template_item().repeated_until(block_end()))
    //             .then_ignore(end_tag().optional())
    //             .map(|(tag, body)| Item::KeywordStatement {
    //                 keyword: tag.keyword,
    //                 statement: tag.statement,
    //                 body,
    //             })
    //     }

    //     fn keyword_tag<'src, F, K>(keyword: F) -> impl Parser<'src, Output = KeywordTag<'src>>
    //     where
    //         F: Fn() -> K,
    //         K: Parser<'src, Output = Keyword>,
    //     {
    //         return select((shorthand_tag(keyword()), longform_tag(keyword())));

    //         fn shorthand_tag<'src>(
    //             keyword: impl Parser<'src, Output = Keyword>,
    //         ) -> impl Parser<'src, Output = KeywordTag<'src>> {
    //             literal("<#")
    //                 .ignore_then(whitespace())
    //                 .ignore_then(keyword)
    //                 .then_ignore(whitespace())
    //                 .then_ignore(literal(">"))
    //                 .map(|k| KeywordTag {
    //                     keyword: k,
    //                     statement: None,
    //                 })
    //         }

    //         fn longform_tag<'src>(
    //             keyword: impl Parser<'src, Output = Keyword>,
    //         ) -> impl Parser<'src, Output = KeywordTag<'src>> {
    //             let end = || whitespace().then(literal("#>"));

    //             literal("<#")
    //                 .ignore_then(whitespace())
    //                 .ignore_then(keyword)
    //                 .then_ignore(whitespace().not_empty())
    //                 .then(take_until(end()).escape("##>", "#>").optional())
    //                 .then_ignore(end())
    //                 .map(|(k, s)| KeywordTag {
    //                     keyword: k,
    //                     statement: s,
    //                 })
    //         }
    //     }
    // }

    // fn plain_statement<'src>() -> impl Parser<'src, Output = Item<'src>> {
    //     let start = || literal("<#").then(whitespace());
    //     let end = || whitespace().then(literal("#>"));

    //     start()
    //         .ignore_then(
    //             take_until(end())
    //                 .escape("##>", "#>")
    //                 .map(Item::PlainStatement),
    //         )
    //         .then_expect_ignore(end())
    // }
}

fn append<'src>(content: &mut Cow<'src, str>, part: &'src str) {
    if content.is_empty() {
        *content = Cow::Borrowed(part);
    } else {
        content.to_mut().push_str(part);
    }
}

fn trim<'src>(content: Cow<'src, str>) -> Cow<'src, str> {
    match content {
        Cow::Borrowed(value) => Cow::Borrowed(value.trim()),
        Cow::Owned(value) => Cow::Owned(value.trim().to_owned()),
    }
}

fn select2<'src>(
    input: &mut Input<'src>,
    parsers: (
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
    ),
) -> ParseResult<'src> {
    let (p1, p2) = parsers;
    if let Some(result) = p1(input)? {
        return Ok(Some(result));
    }

    return p2(input);
}

fn select3<'src>(
    input: &mut Input<'src>,
    parsers: (
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
    ),
) -> ParseResult<'src> {
    let (p1, p2, p3) = parsers;
    if let Some(result) = p1(input)? {
        return Ok(Some(result));
    }

    return select2(input, (p2, p3));
}

#[cfg(test)]
mod tests;
