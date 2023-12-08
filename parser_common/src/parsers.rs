use std::borrow::Cow;

use crate::{literal, take_until, take_while, whitespace, Input, Offset, Parser};

pub fn rust_identifier<'src>() -> impl Parser<'src, Output = Offset<'src>> {
    move |input: &mut Input<'src>| {
        let underscore = || literal("_");
        let xid_start = || take_while(unicode_xid::UnicodeXID::is_xid_start);
        let xid_cont = || take_while(unicode_xid::UnicodeXID::is_xid_continue);

        let result = select! {
            underscore().then(xid_cont().filter(|s| s.len() > 0)),
            xid_start().filter(|s| s.len() > 0).then(xid_cont())
        }
        .parse(input)?;

        Ok(result.map(|(start, rest)| input.combine(&[start, rest])))
    }
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
