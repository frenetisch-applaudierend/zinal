use crate::{Input, ParseResult, Parser};

pub struct Repeated<P>(P);

impl<P> Repeated<P> {
    pub fn new(parser: P) -> Self {
        Self(parser)
    }
}

impl<'src, P> Parser<'src> for Repeated<P>
where
    P: Parser<'src>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
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

impl<'src, P, T> Parser<'src> for RepeatedUntil<P, T>
where
    P: Parser<'src>,
    T: Parser<'src>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
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
