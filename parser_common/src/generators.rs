use crate::{Input, Offset, Parser};

mod func;
mod take_until;
mod whitespace;

pub use func::*;
pub use take_until::*;
pub use whitespace::*;

pub fn literal<'src>(value: &'src str) -> impl Parser<Offset<'src>> {
    move |input: &mut Input<'src>| Ok(input.consume_lit(value))
}

pub fn insert<'src, T: Clone>(value: T) -> impl Parser<T> {
    move |_: &mut Input<'src>| Ok(Some(value.clone()))
}

pub fn todo<'src, T>() -> impl Parser<T> {
    move |_: &mut Input<'src>| Ok(None)
}

pub fn take_while<'src>(predicate: impl Fn(char) -> bool) -> impl Parser<Offset<'src>> {
    move |input: &mut Input<'src>| Ok(Some(input.consume_while(&predicate)))
}

pub fn collect_until<'src, P, PO, S, SO>(parser: P, sentinel: S) -> impl Parser<Vec<PO>>
where
    P: Parser<PO>,
    S: Parser<SO>,
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
