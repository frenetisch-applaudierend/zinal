use crate::{Input, ParseResult, Parser};

#[derive(Debug, Clone)]
pub struct Filter<P, F> {
    parser: P,
    filter: F,
}

impl<P, F> Filter<P, F> {
    pub fn new(parser: P, filter: F) -> Self {
        Self { parser, filter }
    }
}

impl<C, F> Parser for Filter<C, F>
where
    C: Parser,
    F: (Fn(&C::Output) -> bool) + Clone,
{
    type Output = C::Output;

    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        let Some(result) = self.parser.parse(input)? else {
            return Ok(None);
        };

        if (self.filter)(&result) {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}
