use crate::{ParseResult, Parser};

pub fn select<P>(choices: P) -> Select<P> {
    Select(choices)
}

pub struct Select<T>(T);

impl<P1, O> Parser<O> for Select<(P1,)>
where
    P1: Parser<O>,
{
    fn parse<'src>(&self, input: &mut crate::Input<'src>) -> ParseResult<O> {
        self.0 .0.parse(input)
    }
}

impl<P1, P2, O> Parser<O> for Select<(P1, P2)>
where
    P1: Parser<O>,
    P2: Parser<O>,
{
    fn parse<'src>(&self, input: &mut crate::Input<'src>) -> ParseResult<O> {
        if let Some(result) = self.0 .0.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .1.parse(input)? {
            return Ok(Some(result));
        }

        Ok(None)
    }
}

impl<P1, P2, P3, O> Parser<O> for Select<(P1, P2, P3)>
where
    P1: Parser<O>,
    P2: Parser<O>,
    P3: Parser<O>,
{
    fn parse<'src>(&self, input: &mut crate::Input<'src>) -> ParseResult<O> {
        if let Some(result) = self.0 .0.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .1.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .2.parse(input)? {
            return Ok(Some(result));
        }

        Ok(None)
    }
}

impl<P1, P2, P3, P4, O> Parser<O> for Select<(P1, P2, P3, P4)>
where
    P1: Parser<O>,
    P2: Parser<O>,
    P3: Parser<O>,
    P4: Parser<O>,
{
    fn parse<'src>(&self, input: &mut crate::Input<'src>) -> ParseResult<O> {
        if let Some(result) = self.0 .0.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .1.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .2.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .3.parse(input)? {
            return Ok(Some(result));
        }

        Ok(None)
    }
}

impl<P1, P2, P3, P4, P5, O> Parser<O> for Select<(P1, P2, P3, P4, P5)>
where
    P1: Parser<O>,
    P2: Parser<O>,
    P3: Parser<O>,
    P4: Parser<O>,
    P5: Parser<O>,
{
    fn parse<'src>(&self, input: &mut crate::Input<'src>) -> ParseResult<O> {
        if let Some(result) = self.0 .0.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .1.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .2.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .3.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .4.parse(input)? {
            return Ok(Some(result));
        }

        Ok(None)
    }
}

impl<P1, P2, P3, P4, P5, P6, O> Parser<O> for Select<(P1, P2, P3, P4, P5, P6)>
where
    P1: Parser<O>,
    P2: Parser<O>,
    P3: Parser<O>,
    P4: Parser<O>,
    P5: Parser<O>,
    P6: Parser<O>,
{
    fn parse<'src>(&self, input: &mut crate::Input<'src>) -> ParseResult<O> {
        if let Some(result) = self.0 .0.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .1.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .2.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .3.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .4.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .5.parse(input)? {
            return Ok(Some(result));
        }

        Ok(None)
    }
}
