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

impl<'src, P> Parser<'src> for Optional<P>
where
    P: Parser<'src>,
{
    type Output = Option<P::Output>;

    fn parse(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        Ok(Some(self.parser.parse(input)?))
    }
}
