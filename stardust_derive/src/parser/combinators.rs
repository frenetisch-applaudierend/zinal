#![macro_use]

use std::borrow::Cow;

use self::{
    map::Map,
    then::{IgnoreThen, Then, ThenIgnore},
};

use super::{
    error::Error,
    input::{Input, Offset},
};

mod map;
mod then;

pub type ParseResult<T> = Result<Option<T>, Error>;

pub trait Combinator<'src>: Sized {
    type Output;

    fn parse(self, input: &mut Input<'src>) -> ParseResult<Self::Output>;

    fn map<F, U>(self, transform: F) -> Map<Self, F, Self::Output>
    where
        F: Fn(Self::Output) -> U,
    {
        Map::new(self, transform)
    }

    fn then<C>(self, next: C) -> Then<Self, C>
    where
        C: Combinator<'src>,
    {
        Then::new(self, next)
    }

    fn ignore_then<C>(self, next: C) -> IgnoreThen<Self, C>
    where
        C: Combinator<'src>,
    {
        IgnoreThen::new(self, next)
    }

    fn then_ignore<C>(self, next: C) -> ThenIgnore<Self, C>
    where
        C: Combinator<'src>,
    {
        ThenIgnore::new(self, next)
    }
}

impl<'src, T, F> Combinator<'src> for F
where
    F: Fn(&mut Input<'src>) -> ParseResult<T>,
{
    type Output = T;

    fn parse(self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        self(input)
    }
}

pub fn literal<'src>(value: &'static str) -> impl Combinator<'src, Output = Offset<'src>> {
    move |input: &mut Input<'src>| Ok(input.consume_lit(value))
}

pub fn take_until<'a>(terminator: &'a str, escape: &'a str) -> TakeUntil<'a> {
    debug_assert!(
        escape.ends_with(terminator),
        "Currently this combinator requires that the escape ends with the terminator"
    );

    let escape = &escape[..(escape.len() - terminator.len())];

    TakeUntil { terminator, escape }
}

pub struct TakeUntil<'a> {
    terminator: &'a str,
    escape: &'a str,
}

impl<'a, 'src> Combinator<'src> for TakeUntil<'a> {
    type Output = Cow<'src, str>;

    fn parse(self, input: &mut Input<'src>) -> ParseResult<Cow<'src, str>> {
        let mut result = Cow::Borrowed("");

        while !input.is_at_end() {
            let consumed = input
                .consume_until(self.terminator)
                .ok_or(Error::unexpected_eof())?;

            if consumed.ends_with(self.escape) {
                input
                    .consume_lit(self.terminator)
                    .expect("Terminator implied by consume_until");

                result.to_mut().push_str(consumed.as_ref());
                result.to_mut().push_str(self.terminator);
            } else if result.is_empty() {
                return Ok(Some(consumed.into_cow()));
            } else {
                result.to_mut().push_str(consumed.as_ref());
                return Ok(Some(result));
            }
        }

        Err(Error::unexpected_eof())
    }
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
        if let Some(r) = Combinator::parse($c, $i)? {
            return Ok(Some(r));
        }
    };

    ($i:expr => $c:expr, $($cs:expr),+) => {
        _select_inner! { $i => $c }
        _select_inner! { $i => $($cs),+ }
    };
}
