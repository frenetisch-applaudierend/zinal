use std::borrow::Cow;

use proc_macro2::Span;

use crate::parser::{
    common::{parse_rust_identifier, select2},
    input::Offset,
    Keyword, TemplateArgument, TemplateArgumentValue,
};

use super::{
    common::{parse_rust_typename, select6, ParseResult},
    input::Input,
    Item,
};

pub struct HtmlParser;

impl HtmlParser {
    pub fn parse<'src>(&mut self, mut input: Input<'src>) -> Result<Vec<Item<'src>>, syn::Error> {
        let mut items = Vec::new();

        while !input.is_at_end() {
            items.push(parse_template_item(&mut input)?.expect("Should never be None"));
        }

        Ok(items)
    }
}

fn parse_template_item<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    select6(
        input,
        (
            parse_escape,
            parse_expression,
            parse_statement,
            parse_comment,
            parse_child_template,
            parse_literal,
        ),
    )
}

fn parse_escape<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    if input.consume_lit("%{{").is_some() {
        return Ok(Some(Item::Literal(Cow::from("{{"))));
    }

    if input.consume_lit("%<#").is_some() {
        return Ok(Some(Item::Literal(Cow::from("<#"))));
    }

    Ok(None)
}

fn parse_expression<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    if input.consume_lit("{{").is_none() {
        return Ok(None);
    }

    let mut content = Cow::<'src, str>::Borrowed("");

    loop {
        let part = input.consume_until_any("}%");

        append(&mut content, part.into_str());

        if input.consume_lit("%}}").is_some() {
            append(&mut content, "}}");
        } else if input.consume_lit("}}").is_some() {
            break;
        } else if input.consume_lit("}").is_some() {
            append(&mut content, "}");
        } else if input.consume_lit("%").is_some() {
            append(&mut content, "%");
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

        if input.consume_lit("<#").is_none() {
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

        fn parse_keyword(input: &mut Input<'_>) -> Option<Keyword> {
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
        if input.consume_lit("<#").is_none() {
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
            let part = input.consume_until_any("#%");

            append(&mut content, part.into_str());

            if input.consume_lit("%#>").is_some() {
                append(&mut content, "#>");
            } else if input.consume_lit("#>").is_some() {
                break;
            } else if input.consume_lit("#").is_some() {
                append(&mut content, "#");
            } else if input.consume_lit("%").is_some() {
                append(&mut content, "%");
            } else {
                return Err(syn::Error::new(Span::call_site(), "Unterminated statement"));
            }
        }

        Ok(trim(content))
    }
}

fn parse_comment<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    let Some(start) = input.consume_lit("<!--") else {
        return Ok(None);
    };

    let content = input.consume_until("-->");

    let Some(end) = input.consume_lit("-->") else {
        return Err(syn::Error::new(Span::call_site(), "Unterminated comment"));
    };

    let comment = input.combine(&[start, content, end]);

    Ok(Some(Item::Literal(comment.into_cow())))
}

fn parse_child_template<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    let position = input.position();

    if input.consume_lit("<").is_none() {
        return Ok(None);
    }

    input.consume_while(char::is_whitespace);

    let Some(name) = parse_rust_typename(input).map(Offset::into_cow) else {
        input.reset_to(position);
        return Ok(None);
    };

    if !name.contains("::") && !name.contains(char::is_uppercase) {
        input.reset_to(position);
        return Ok(None);
    }

    let mut arguments = Vec::new();
    loop {
        let whitespace = input.consume_while(char::is_whitespace);
        if whitespace.is_empty() {
            break;
        }

        let Some(argument) = parse_template_argument(input)? else {
            break;
        };

        arguments.push(argument);
    }

    input.consume_while(char::is_whitespace);

    let children = if input.consume_lit("/>").is_some() {
        // Collapsed tag
        Vec::new()
    } else if input.consume_lit(">").is_some() {
        // Regular tag with body - parse_template_children also parses the end tag
        parse_template_children(input, name.as_ref())?
    } else {
        return Err(syn::Error::new(
            Span::call_site(),
            "Unterminated template reference",
        ));
    };

    return Ok(Some(Item::ChildTemplate {
        name,
        arguments,
        children,
    }));

    fn parse_template_argument<'src>(
        input: &mut Input<'src>,
    ) -> Result<Option<TemplateArgument<'src>>, syn::Error> {
        let Some(name) = parse_rust_identifier(input).map(Offset::into_cow) else {
            return Ok(None);
        };

        input.consume_while(char::is_whitespace);
        if input.consume_lit("=").is_none() {
            return Ok(Some(TemplateArgument {
                name,
                value: TemplateArgumentValue::BoolLiteral(true),
            }));
        }
        input.consume_while(char::is_whitespace);

        let value = parse_template_argument_value(input)?;

        return Ok(Some(TemplateArgument { name, value }));

        fn parse_template_argument_value<'src>(
            input: &mut Input<'src>,
        ) -> Result<TemplateArgumentValue<'src>, syn::Error> {
            if input.consume_lit("true").is_some() {
                return Ok(TemplateArgumentValue::BoolLiteral(true));
            }

            if input.consume_lit("false").is_some() {
                return Ok(TemplateArgumentValue::BoolLiteral(false));
            }

            if let Some(value) = parse_expression_value(input)? {
                return Ok(value);
            }

            if let Some(value) = parse_double_ticks_value(input)? {
                return Ok(value);
            }

            if let Some(value) = parse_single_ticks_value(input)? {
                return Ok(value);
            }

            return Err(syn::Error::new(
                Span::call_site(),
                "Unexpected token for template argument value",
            ));

            fn parse_expression_value<'src>(
                input: &mut Input<'src>,
            ) -> Result<Option<TemplateArgumentValue<'src>>, syn::Error> {
                let Some(Item::Expression(expr)) = parse_expression(input)? else {
                    return Ok(None);
                };

                Ok(Some(TemplateArgumentValue::Expression(expr)))
            }

            fn parse_double_ticks_value<'src>(
                input: &mut Input<'src>,
            ) -> Result<Option<TemplateArgumentValue<'src>>, syn::Error> {
                if input.consume_lit("\"").is_none() {
                    return Ok(None);
                }

                let mut content = Cow::<'src, str>::Borrowed("");

                loop {
                    let part = input.consume_until_any("\\\"");

                    append(&mut content, part.into_str());

                    if input.consume_lit("\\\"").is_some() {
                        append(&mut content, "\"");
                    } else if input.consume_lit("\"").is_some() {
                        break;
                    } else {
                        return Err(syn::Error::new(
                            Span::call_site(),
                            "Unterminated string literal",
                        ));
                    }
                }

                Ok(Some(TemplateArgumentValue::StrLiteral(content)))
            }

            fn parse_single_ticks_value<'src>(
                input: &mut Input<'src>,
            ) -> Result<Option<TemplateArgumentValue<'src>>, syn::Error> {
                if input.consume_lit("'").is_none() {
                    return Ok(None);
                }

                let mut content = Cow::<'src, str>::Borrowed("");

                loop {
                    let part = input.consume_until_any("\\'");

                    append(&mut content, part.into_str());

                    if input.consume_lit("\\'").is_some() {
                        append(&mut content, "'");
                    } else if input.consume_lit("'").is_some() {
                        break;
                    } else {
                        return Err(syn::Error::new(
                            Span::call_site(),
                            "Unterminated string literal",
                        ));
                    }
                }

                Ok(Some(TemplateArgumentValue::StrLiteral(content)))
            }
        }
    }

    fn parse_template_children<'src>(
        input: &mut Input<'src>,
        name: &str,
    ) -> Result<Vec<Item<'src>>, syn::Error> {
        let mut children = Vec::new();

        while !input.is_at_end() {
            if parse_end_tag(input, name)? {
                return Ok(children);
            }

            let Some(item) = parse_template_item(input)? else {
                break;
            };

            children.push(item);
        }

        return Err(syn::Error::new(
            Span::call_site(),
            "Unterminated template reference",
        ));

        fn parse_end_tag(input: &mut Input<'_>, name: &str) -> Result<bool, syn::Error> {
            let position = input.position();

            if input.consume_lit("</").is_none() {
                return Ok(false);
            }

            input.consume_while(char::is_whitespace);

            if input.consume_lit(name).is_none() {
                input.reset_to(position);
                return Ok(false);
            }

            input.consume_while(char::is_whitespace);

            if input.consume_lit(">").is_none() {
                input.reset_to(position);
                return Ok(false);
            }

            Ok(true)
        }
    }
}

fn parse_literal<'src>(input: &mut Input<'src>) -> ParseResult<'src> {
    let first = input
        .consume_count(1)
        .expect("This method must not be called with an empty input");

    let rest = input.consume_until_any("<{%");

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

fn trim(content: Cow<'_, str>) -> Cow<'_, str> {
    match content {
        Cow::Borrowed(value) => Cow::Borrowed(value.trim()),
        Cow::Owned(value) => Cow::Owned(value.trim().to_owned()),
    }
}

#[cfg(test)]
mod tests;
