use std::borrow::Cow;

use crate::{literal, parser, take_until, whitespace, Offset, Parser};

pub fn rust_identifier<'src>() -> impl Parser<'src, Output = Offset<'src>> {
    parser(|_| todo!())()
}

pub fn embedded_code<'src>(
    start: &'static str,
    end: &'static str,
    escape: &'static str,
) -> impl Parser<'src, Output = Cow<'src, str>> {
    literal(start)
        .ignore_then(whitespace().optional())
        .ignore_then(take_until(whitespace().optional().then(literal(end))).escape(escape, end))
        .then_ignore(literal(end))
}
