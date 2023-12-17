use crate::{Input, ParseError, ParseResult, Parser};

pub struct Then<P1, P2> {
    parser1: P1,
    parser1_expected: bool,
    parser2: P2,
    parser2_expected: bool,
}

pub struct IgnoreThen<P1, P2> {
    then: Then<P1, P2>,
}

pub struct ThenIgnore<P1, P2> {
    then: Then<P1, P2>,
}

impl<P1, P2> Then<P1, P2> {
    pub fn new(parser1: P1, parser1_expected: bool, parser2: P2, parser2_expected: bool) -> Self {
        Self {
            parser1,
            parser1_expected,
            parser2,
            parser2_expected,
        }
    }
}

impl<P1, P2> IgnoreThen<P1, P2> {
    pub fn new(parser1: P1, parser2: P2) -> Self {
        Self {
            then: Then::new(parser1, false, parser2, false),
        }
    }
}

impl<P1, P2> ThenIgnore<P1, P2> {
    pub fn new(parser1: P1, parser2: P2, expected: bool) -> Self {
        Self {
            then: Then::new(parser1, false, parser2, expected),
        }
    }
}

impl<P1, P2> Parser for Then<P1, P2>
where
    P1: Parser,
    P2: Parser,
{
    type Output = (P1::Output, P2::Output);

    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        let position = input.position();

        let Some(result1) = self.parser1.parse(input)? else {
            if self.parser1_expected {
                return Err(ParseError::new("Expected content not found"));
            }
            input.reset_to(position);
            return Ok(None);
        };

        let Some(result2) = self.parser2.parse(input)? else {
            if self.parser2_expected {
                return Err(ParseError::new("Expected content not found"));
            }
            input.reset_to(position);
            return Ok(None);
        };

        Ok(Some((result1, result2)))
    }
}

impl<P1, P2> Parser for IgnoreThen<P1, P2>
where
    P1: Parser,
    P2: Parser,
{
    type Output = P2::Output;

    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        Ok(self.then.parse(input)?.map(|(_, r)| r))
    }
}

impl<P1, P2> Parser for ThenIgnore<P1, P2>
where
    P1: Parser,
    P2: Parser,
{
    type Output = P1::Output;

    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        Ok(self.then.parse(input)?.map(|(r, _)| r))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser, Input, Parser};

    #[test]
    fn then() {
        let first = parser(|_| Ok(Some(10)));
        let second = parser(|_| Ok(Some(20)));

        let combined = first().then(second());

        let result = combined.parse(&mut Input::new(""));

        assert_eq!(Ok(Some((10, 20))), result);
    }
}
