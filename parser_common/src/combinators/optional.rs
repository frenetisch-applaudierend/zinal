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

impl<P, O> Parser<Option<O>> for Optional<P>
where
    P: Parser<O>,
{
    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Option<O>> {
        Ok(Some(self.parser.parse(input)?))
    }
}
