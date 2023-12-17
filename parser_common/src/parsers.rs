use std::borrow::Cow;

use crate::{literal, select, take_until, take_while, whitespace, Input, Offset, Parser};

pub fn rust_identifier<'src>() -> impl Parser<Output = Offset<'src>> {
    move |input: &mut Input<'src>| {
        let underscore = || literal("_");
        let xid_start = || take_while(unicode_xid::UnicodeXID::is_xid_start);
        let xid_cont = || take_while(unicode_xid::UnicodeXID::is_xid_continue);

        let result = select((
            underscore().then(xid_cont().filter(|s| s.len() > 0)),
            xid_start().filter(|s| s.len() > 0).then(xid_cont()),
        ))
        .parse(input)?;

        Ok(result.map(|(start, rest)| input.combine(&[start, rest])))
    }
}
