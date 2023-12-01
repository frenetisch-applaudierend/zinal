use std::marker::PhantomData;

use crate::parser::input::Input;

use super::{Combinator, ParseResult};

#[derive(Debug, Clone)]
pub struct Map<C, F, T> {
    combinator: C,
    transform: F,
    _phantom: PhantomData<T>,
}

impl<C, F, T> Map<C, F, T> {
    pub fn new(combinator: C, transform: F) -> Self {
        Self {
            combinator,
            transform,
            _phantom: PhantomData,
        }
    }
}

impl<'src, C, F, T, U> Combinator<'src> for Map<C, F, T>
where
    C: Combinator<'src, Output = T>,
    F: Fn(T) -> U,
{
    type Output = U;

    fn parse(self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        match self.combinator.parse(input)? {
            Some(r) => Ok(Some((self.transform)(r))),
            None => Ok(None),
        }
    }
}
