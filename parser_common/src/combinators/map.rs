use std::marker::PhantomData;

use crate::{Input, ParseResult, Parser};

pub struct Map<P, F, T> {
    parser: P,
    transform: F,
    _phantom: PhantomData<T>,
}

impl<P, F, T> Map<P, F, T> {
    pub fn new(parser: P, transform: F) -> Self {
        Self {
            parser,
            transform,
            _phantom: PhantomData,
        }
    }
}

impl<'src, P, F, T, U> Parser<'src> for Map<P, F, T>
where
    P: Parser<'src, Output = T>,
    F: Fn(T) -> U,
{
    type Output = U;

    fn parse(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        match self.parser.parse(input)? {
            Some(orig) => Ok(Some((self.transform)(orig))),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser, Input, Parser};

    #[test]
    fn map() {
        let mut input = Input::new("");
        let producer = parser(|_: &mut Input<'_>| Ok(Some(10)));

        let mapped = producer().map(|x| x * 2);

        assert_eq!(Ok(Some(20)), mapped.parse(&mut input))
    }
}
