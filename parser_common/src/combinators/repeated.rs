use std::marker::PhantomData;

use crate::{Input, ParseResult, Parser};

pub struct Repeated<P>(P);

impl<P> Repeated<P> {
    pub fn new(parser: P) -> Self {
        Self(parser)
    }
}

impl<P, O> Parser<Vec<O>> for Repeated<P>
where
    P: Parser<O>,
{
    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Vec<O>> {
        let mut results = Vec::new();

        while !input.is_at_end() {
            match self.0.parse(input)? {
                Some(result) => results.push(result),
                _ => break,
            };
        }

        Ok(Some(results))
    }
}

pub struct RepeatedUntil<P, T, TO> {
    parser: P,
    terminator: T,
    _marker: PhantomData<TO>,
}

impl<P, T, TO> RepeatedUntil<P, T, TO> {
    pub fn new(parser: P, terminator: T) -> Self {
        Self {
            parser,
            terminator,
            _marker: PhantomData,
        }
    }
}

impl<P, T, TO, O> Parser<Vec<O>> for RepeatedUntil<P, T, TO>
where
    P: Parser<O>,
    T: Parser<TO>,
{
    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Vec<O>> {
        let mut results = Vec::new();
        let mut snapshot = input.position();

        while !input.is_at_end() {
            if self.terminator.parse(input)?.is_some() {
                input.reset_to(snapshot);
                break;
            }

            match self.parser.parse(input)? {
                Some(result) => results.push(result),
                _ => break,
            };

            snapshot = input.position();
        }

        Ok(Some(results))
    }
}
