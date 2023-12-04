use crate::{Input, ParseResult, Parser};

pub fn parser<'src, F, T>(func: F) -> impl Fn() -> FuncParser<F>
where
    F: Fn(&mut Input<'src>) -> ParseResult<T> + Clone,
{
    move || FuncParser(func.clone())
}

pub struct FuncParser<F>(F);

impl<'src, F, T> Parser<'src> for FuncParser<F>
where
    F: Fn(&mut Input<'src>) -> ParseResult<T>,
{
    type Output = T;

    fn parse(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        self.0.parse(input)
    }
}

impl<'src, Func, T> Parser<'src> for Func
where
    Func: Fn(&mut Input<'src>) -> ParseResult<T>,
{
    type Output = T;

    fn parse(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        self(input)
    }
}
