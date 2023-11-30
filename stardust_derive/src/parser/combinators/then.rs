use crate::parser::input::Input;

use super::{Combinator, ParseResult};

pub struct Then<C1, C2> {
    combinator1: C1,
    combinator2: C2,
}

pub struct IgnoreThen<C1, C2> {
    then: Then<C1, C2>,
}

pub struct ThenIgnore<C1, C2> {
    then: Then<C1, C2>,
}

impl<C1, C2> Then<C1, C2> {
    pub fn new(combinator1: C1, combinator2: C2) -> Self {
        Self {
            combinator1,
            combinator2,
        }
    }
}

impl<C1, C2> IgnoreThen<C1, C2> {
    pub fn new(combinator1: C1, combinator2: C2) -> Self {
        Self {
            then: Then::new(combinator1, combinator2),
        }
    }
}

impl<C1, C2> ThenIgnore<C1, C2> {
    pub fn new(combinator1: C1, combinator2: C2) -> Self {
        Self {
            then: Then::new(combinator1, combinator2),
        }
    }
}

impl<'src, C1, C2> Combinator<'src> for Then<C1, C2>
where
    C1: Combinator<'src>,
    C2: Combinator<'src>,
{
    type Output = (C1::Output, C2::Output);

    fn parse(self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        let position = input.position();

        let Some(result1) = self.combinator1.parse(input)? else {
            input.reset_to(position);
            return Ok(None);
        };

        let Some(result2) = self.combinator2.parse(input)? else {
            input.reset_to(position);
            return Ok(None);
        };

        Ok(Some((result1, result2)))
    }
}

impl<'src, C1, C2> Combinator<'src> for IgnoreThen<C1, C2>
where
    C1: Combinator<'src>,
    C2: Combinator<'src>,
{
    type Output = C2::Output;

    fn parse(self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        self.then.map(|(_, r)| r).parse(input)
    }
}

impl<'src, C1, C2> Combinator<'src> for ThenIgnore<C1, C2>
where
    C1: Combinator<'src>,
    C2: Combinator<'src>,
{
    type Output = C1::Output;

    fn parse(self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        self.then.map(|(r, _)| r).parse(input)
    }
}
