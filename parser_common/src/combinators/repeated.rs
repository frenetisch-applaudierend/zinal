use crate::{Input, ParseResult, Parser};

pub struct Repeated<P>(P);

impl<P> Repeated<P> {
    pub fn new(parser: P) -> Self {
        Self(parser)
    }
}

impl<P> Parser for Repeated<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;

    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
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

pub struct RepeatedUntil<P, T> {
    parser: P,
    terminator: T,
}

impl<P, T> RepeatedUntil<P, T> {
    pub fn new(parser: P, terminator: T) -> Self {
        Self { parser, terminator }
    }
}

impl<P, T> Parser for RepeatedUntil<P, T>
where
    P: Parser,
    T: Parser,
{
    type Output = Vec<P::Output>;

    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
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
