#![macro_use]

use std::marker::PhantomData;

use super::{error::Error, input::Input};

pub type ParseResult<T> = Result<Option<T>, Error>;

pub trait Combinator<'src, T>: Sized {
    fn parse(&self, input: &mut Input<'src>) -> ParseResult<T>;

    fn map<F, U>(self, transform: F) -> Map<Self, F, T>
    where
        F: Fn(T) -> U,
    {
        map(self, transform)
    }
}

impl<'src, T, F> Combinator<'src, T> for F
where
    F: Fn(&mut Input<'src>) -> ParseResult<T>,
{
    fn parse(&self, input: &mut Input<'src>) -> ParseResult<T> {
        self(input)
    }
}

pub struct Map<C, F, T> {
    transform: F,
    combinator: C,
    _phantom: PhantomData<T>,
}

impl<'src, C, F, T, U> Combinator<'src, U> for Map<C, F, T>
where
    C: Combinator<'src, T>,
    F: Fn(T) -> U,
{
    fn parse(&self, input: &mut Input<'src>) -> ParseResult<U> {
        match self.combinator.parse(input)? {
            Some(r) => Ok(Some((self.transform)(r))),
            None => Ok(None),
        }
    }
}

pub fn map<'src, C, F, T, U>(combinator: C, transform: F) -> Map<C, F, T>
where
    C: Combinator<'src, T>,
    F: Fn(T) -> U,
{
    Map {
        combinator,
        transform,
        _phantom: PhantomData,
    }
}

pub fn literal<'src>(value: &'static str) -> impl Combinator<'src, &'src str> {
    move |input: &mut Input<'src>| Ok(input.try_consume(value))
}

macro_rules! select {
    ($($cs:expr),+) => {
        |input: &mut Input<'src>| {
            _select_inner! { input => $($cs),+ }

            Ok(None)
        }
    };
}

macro_rules! _select_inner {
    ($i:expr => $c:expr) => {
        if let Some(r) = Combinator::parse(&$c, $i)? {
            return Ok(Some(r));
        }
    };

    ($i:expr => $c:expr, $($cs:expr),+) => {
        _select_inner! { $i => $c }
        _select_inner! { $i => $($cs),+ }
    };
}
