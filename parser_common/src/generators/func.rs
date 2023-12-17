use std::marker::PhantomData;

use crate::{Input, ParseResult, Parser};

pub fn parser<'src, F, O>(func: F) -> impl Fn() -> FuncParser<F, O>
where
    F: Fn(&mut Input<'src>) -> ParseResult<O> + Clone,
{
    move || FuncParser(func.clone(), PhantomData)
}

pub struct FuncParser<F, O>(F, PhantomData<O>);

impl<F, O> Parser<O> for FuncParser<F, O>
where
    for<'src> F: Fn(&mut Input<'src>) -> ParseResult<O>,
{
    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<O> {
        self.0.parse(input)
    }
}

impl<Func, O> Parser<O> for Func
where
    for<'src> Func: Fn(&mut Input<'src>) -> ParseResult<O>,
{
    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<O> {
        self(input)
    }
}
