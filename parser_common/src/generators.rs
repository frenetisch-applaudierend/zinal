use crate::{Input, Offset, Parser};

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
