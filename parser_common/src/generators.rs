use crate::{Input, Offset, ParseError, Parser};

mod func;
mod take_until;

pub use func::*;
pub use take_until::*;

pub fn literal<'src>(value: &'src str) -> impl Parser<'src, Output = Offset<'src>> {
    move |input: &mut Input<'src>| Ok(input.consume_lit(value))
}

pub fn whitespace<'src>() -> impl Parser<'src, Output = Offset<'src>> {
    move |input: &mut Input<'src>| Ok(input.consume_while(char::is_whitespace))
}

pub fn insert<'src, T: Clone>(value: T) -> impl Parser<'src, Output = T> {
    move |_: &mut Input<'src>| Ok(Some(value.clone()))
}

pub fn take_while<'src>(
    predicate: impl Fn(char) -> bool,
) -> impl Parser<'src, Output = Offset<'src>> {
    move |input: &mut Input<'src>| Ok(input.consume_while(&predicate))
}

pub fn collect_until<'src, P, S>(
    parser: P,
    sentinel: S,
) -> impl Parser<'src, Output = Vec<P::Output>>
where
    P: Parser<'src>,
    S: Parser<'src>,
{
    move |input: &mut Input<'src>| {
        let mut output = vec![];
        let start = input.position();

        while !input.is_at_end() {
            if sentinel.peek(input)?.is_some() {
                return Ok(Some(output));
            }

            let Some(elem) = parser.parse(input)? else {
                input.reset_to(start);
                return Ok(None);
            };

            output.push(elem);
        }

        Ok(None)
    }
}
