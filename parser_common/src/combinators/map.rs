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

impl<P, F, T, U> Parser<U> for Map<P, F, T>
where
    P: Parser<T>,
    F: Fn(T) -> U,
{
    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<U> {
        match self.parser.parse(input)? {
            Some(orig) => Ok(Some((self.transform)(orig))),
            None => Ok(None),
        }
    }
}

pub struct To<P, PO, T> {
    parser: P,
    value: T,
    _marker: PhantomData<PO>,
}

impl<P: Parser<PO>, PO, T> To<P, PO, T> {
    pub fn new(parser: P, value: T) -> Self {
        Self {
            parser,
            value,
            _marker: PhantomData,
        }
    }
}

impl<P, PO, T> Parser<T> for To<P, PO, T>
where
    P: Parser<PO>,
    T: Clone,
{
    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<T> {
        match self.parser.parse(input)? {
            Some(_) => Ok(Some(self.value.clone())),
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
