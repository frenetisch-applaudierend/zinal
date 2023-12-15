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
