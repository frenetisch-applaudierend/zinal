use crate::parser::input::Input;

use super::{Combinator, ParseResult};

#[derive(Debug, Clone)]
pub struct Filter<C, F> {
    combinator: C,
    filter: F,
}

impl<C, F> Filter<C, F> {
    pub fn new(combinator: C, filter: F) -> Self {
        Self { combinator, filter }
    }
}

impl<'src, C, F> Combinator<'src> for Filter<C, F>
where
    C: Combinator<'src>,
    F: (Fn(&C::Output) -> bool) + Clone,
{
    type Output = C::Output;

    fn parse(self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        let Some(result) = self.combinator.parse(input)? else {
            return Ok(None);
        };

        if (self.filter)(&result) {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}
