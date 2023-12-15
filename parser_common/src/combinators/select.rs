use crate::Parser;

pub fn select<P>(choices: P) -> Select<P> {
    Select(choices)
}

pub struct Select<T>(T);

impl<'src, P1> Parser<'src> for Select<(P1,)>
where
    P1: Parser<'src>,
{
    type Output = P1::Output;

    fn parse(&self, input: &mut crate::Input<'src>) -> crate::ParseResult<Self::Output> {
        self.0 .0.parse(input)
    }
}

impl<'src, P1, P2> Parser<'src> for Select<(P1, P2)>
where
    P1: Parser<'src>,
    P2: Parser<'src, Output = P1::Output>,
{
    type Output = P1::Output;

    fn parse(&self, input: &mut crate::Input<'src>) -> crate::ParseResult<Self::Output> {
        if let Some(result) = self.0 .0.parse(input)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.0 .1.parse(input)? {
            return Ok(Some(result));
        }

        Ok(None)
    }
}

impl<'src, P1, P2, P3> Parser<'src> for Select<(P1, P2, P3)>
where
    P1: Parser<'src>,
    P2: Parser<'src, Output = P1::Output>,
    P3: Parser<'src, Output = P1::Output>,
{
    type Output = P1::Output;

    fn parse(&self, input: &mut crate::Input<'src>) -> crate::ParseResult<Self::Output> {
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

impl<'src, P1, P2, P3, P4> Parser<'src> for Select<(P1, P2, P3, P4)>
where
    P1: Parser<'src>,
    P2: Parser<'src, Output = P1::Output>,
    P3: Parser<'src, Output = P1::Output>,
    P4: Parser<'src, Output = P1::Output>,
{
    type Output = P1::Output;

    fn parse(&self, input: &mut crate::Input<'src>) -> crate::ParseResult<Self::Output> {
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

impl<'src, P1, P2, P3, P4, P5> Parser<'src> for Select<(P1, P2, P3, P4, P5)>
where
    P1: Parser<'src>,
    P2: Parser<'src, Output = P1::Output>,
    P3: Parser<'src, Output = P1::Output>,
    P4: Parser<'src, Output = P1::Output>,
    P5: Parser<'src, Output = P1::Output>,
{
    type Output = P1::Output;

    fn parse(&self, input: &mut crate::Input<'src>) -> crate::ParseResult<Self::Output> {
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
