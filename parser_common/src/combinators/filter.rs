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

impl<C, F, O> Parser<O> for Filter<C, F>
where
    C: Parser<O>,
    F: (Fn(&O) -> bool) + Clone,
{
    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<O> {
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
