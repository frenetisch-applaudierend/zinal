use crate::{Input, ParseResult, Parser};

#[derive(Debug, Clone)]
pub struct Optional<C> {
    parser: C,
}

impl<P> Optional<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<P> Parser for Optional<P>
where
    P: Parser,
{
    type Output = Option<P::Output>;

    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        Ok(Some(self.parser.parse(input)?))
    }
}
