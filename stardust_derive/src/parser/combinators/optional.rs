use crate::parser::input::Input;

use super::{Combinator, ParseResult};

#[derive(Debug, Clone)]
pub struct Optional<C> {
    combinator: C,
}

impl<C> Optional<C> {
    pub fn new(combinator: C) -> Self {
        Self { combinator }
    }
}

impl<'src, C> Combinator<'src> for Optional<C>
where
    C: Combinator<'src>,
{
    type Output = Option<C::Output>;

    fn parse(self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        Ok(Some(self.combinator.parse(input)?))
    }
}
