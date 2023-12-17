mod combinators;
mod generators;
mod input;
mod parsers;

use std::borrow::Cow;

pub use combinators::*;
pub use generators::*;
pub use input::*;
pub use parsers::*;

pub trait Parser<O> {
    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<O>;

    fn peek<'src>(&self, input: &mut Input<'src>) -> ParseResult<O> {
        let position = input.position();
        let result = self.parse(input);
        input.reset_to(position);
        result
    }

    fn boxed(self) -> Boxed<O>
    where
        Self: Sized + 'static,
    {
        Boxed::new(self)
    }

    fn map<F, U>(self, transform: F) -> Map<Self, F, O>
    where
        Self: Sized,
        F: Fn(O) -> U,
    {
        Map::new(self, transform)
    }

    fn to<U: Clone>(self, value: U) -> To<Self, O, U>
    where
        Self: Sized,
    {
        To::new(self, value)
    }

    fn filter<F>(self, filter: F) -> Filter<Self, F>
    where
        Self: Sized,
        F: Fn(&O) -> bool,
    {
        Filter::new(self, filter)
    }

    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional::new(self)
    }

    fn then<P, PO>(self, next: P) -> Then<Self, P>
    where
        Self: Sized,
        P: Parser<PO>,
    {
        Then::new(self, false, next, false)
    }

    fn ignore_then<P, PO>(self, next: P) -> IgnoreThen<Self, P, O>
    where
        Self: Sized,
        P: Parser<PO>,
    {
        IgnoreThen::new(self, next)
    }

    fn then_ignore<P, PO>(self, next: P) -> ThenIgnore<Self, P, PO>
    where
        Self: Sized,
        P: Parser<PO>,
    {
        ThenIgnore::new(self, next, false)
    }

    fn then_expect_ignore<P, PO>(self, next: P) -> ThenIgnore<Self, P, PO>
    where
        Self: Sized,
        P: Parser<PO>,
    {
        ThenIgnore::new(self, next, true)
    }

    fn repeated(self) -> Repeated<Self>
    where
        Self: Sized,
    {
        Repeated::new(self)
    }

    fn repeated_until<T, TO>(self, terminator: T) -> RepeatedUntil<Self, T, TO>
    where
        Self: Sized,
        T: Parser<TO>,
    {
        RepeatedUntil::new(self, terminator)
    }
}

pub type ParseResult<T> = Result<Option<T>, ParseError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    message: Cow<'static, str>,
}

impl ParseError {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}
