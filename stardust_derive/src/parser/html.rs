use std::borrow::Cow;

use proc_macro2::Span;

use crate::parser::Keyword;

use super::{input::Input, Item, TemplateParser};

pub struct HtmlParser;

type ParseResult<'src> = Result<Option<Item<'src>>, syn::Error>;

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
    select5(
        input,
        (
            parse_escape,
            parse_expression,
            parse_statement,
            parse_child_template,
            parse_literal,
        ),
    )
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
        let part = input.consume_until("}");

        append(&mut content, part.into_str());

        if input.consume_lit("}}").is_some() {
            append(&mut content, "}");
        } else if input.consume_lit("}").is_some() {
            break;
        } else {
            return Err(syn::Error::new(
                Span::call_site(),
                "Unterminated expression",
            ));
        }
    }

    Ok(Some(Item::Expression(trim(content))))
}

fn parse_statement<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    return select2(input, (parse_keyword_statement, parse_plain_statement));

    fn parse_keyword_statement<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
        let position = input.position();

        if !input.consume_lit("<#").is_some() {
            return Ok(None);
        }

        input.consume_while(char::is_whitespace);

        let Some(keyword) = parse_keyword(input) else {
            input.reset_to(position);
            return Ok(None);
        };

        let whitespace = input.consume_while(char::is_whitespace);

        let statement = if input
            .consume_lit(">")
            .or_else(|| input.consume_lit("#>"))
            .is_some()
        {
            // Shorthand form
            None
        } else {
            // Long form

            if whitespace.len() < 1 {
                // In long form at least one whitespace is needed to separate
                // the keyword from the statement content
                input.reset_to(position);
                return Ok(None);
            }
            Some(parse_statement_content(input)?)
        };

        let body = if keyword.has_body() {
            parse_body(input)?
        } else {
            Vec::new()
        };

        return Ok(Some(Item::KeywordStatement {
            keyword,
            statement,
            body,
        }));

        fn parse_keyword<'src>(input: &mut Input<'src>) -> Option<Keyword> {
            if input.consume_lit("if").is_some() {
                return Some(Keyword::If);
            }

            if input.consume_lit("else").is_some() {
                let position = input.position();
                input.consume_while(char::is_whitespace);

                if input.consume_lit("if").is_some() {
                    return Some(Keyword::ElseIf);
                } else {
                    input.reset_to(position);
                    return Some(Keyword::Else);
                }
            }

            if input.consume_lit("for").is_some() {
                return Some(Keyword::For);
            }

            if input.consume_lit("while").is_some() {
                return Some(Keyword::While);
            }

            if input.consume_lit("loop").is_some() {
                return Some(Keyword::Loop);
            }

            if input.consume_lit("end").is_some() {
                return Some(Keyword::End);
            }

            if input.consume_lit("break").is_some() {
                return Some(Keyword::Break);
            }

            if input.consume_lit("continue").is_some() {
                return Some(Keyword::Continue);
            }

            if input.consume_lit("let").is_some() {
                return Some(Keyword::Let);
            }

            None
        }

        fn parse_body<'src>(input: &mut Input<'src>) -> Result<Vec<Item<'src>>, syn::Error> {
            let mut body = Vec::new();

            while !input.is_at_end() {
                let position = input.position();

                let item = parse_template_item(input)?
                    .expect("parse_template_item should never return None");

                match item {
                    Item::KeywordStatement {
                        keyword,
                        statement: _,
                        body: _,
                    } if keyword.is_block_terminator() => {
                        if keyword != Keyword::End {
                            input.reset_to(position);
                        }
                        break;
                    }
                    _ => body.push(item),
                }
            }

            Ok(body)
        }
    }

    fn parse_plain_statement<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
        if !input.consume_lit("<#").is_some() {
            return Ok(None);
        }

        let content = parse_statement_content(input)?;

        Ok(Some(Item::PlainStatement(content)))
    }

    fn parse_statement_content<'src>(
        input: &mut Input<'src>,
    ) -> Result<Cow<'src, str>, syn::Error> {
        let mut content = Cow::<'src, str>::Borrowed("");

        loop {
            let part = input.consume_until("#");

            append(&mut content, part.into_str());

            if input.consume_lit("##>").is_some() {
                append(&mut content, "#>");
            } else if input.consume_lit("#>").is_some() {
                break;
            } else if input.consume_lit("#").is_some() {
                append(&mut content, "#");
            } else {
                return Err(syn::Error::new(Span::call_site(), "Unterminated statement"));
            }
        }

        Ok(trim(content))
    }
}

fn parse_child_template<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    Ok(None)
}

fn parse_literal<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    let first = input
        .consume_count(1)
        .expect("This method must not be called with an empty input");

    let rest = input.consume_until_any("<{");

    let combined = input.combine(&[first, rest]).into_cow();

    Ok(Some(Item::Literal(combined)))
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

fn select4<'src>(
    input: &mut Input<'src>,
    parsers: (
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
    ),
) -> ParseResult<'src> {
    let (p1, p2, p3, p4) = parsers;
    if let Some(result) = p1(input)? {
        return Ok(Some(result));
    }

    return select3(input, (p2, p3, p4));
}

fn select5<'src>(
    input: &mut Input<'src>,
    parsers: (
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
    ),
) -> ParseResult<'src> {
    let (p1, p2, p3, p4, p5) = parsers;
    if let Some(result) = p1(input)? {
        return Ok(Some(result));
    }

    return select4(input, (p2, p3, p4, p5));
}

#[cfg(test)]
mod tests;
