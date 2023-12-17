mod combinators;
mod generators;
mod input;
mod parsers;

use std::borrow::Cow;

pub use combinators::*;
pub use generators::*;
pub use input::*;
pub use parsers::*;

pub trait Parser {
    type Output;

    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output>;

    fn peek<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        let position = input.position();
        let result = self.parse(input);
        input.reset_to(position);
        result
    }

    fn boxed(self) -> Boxed<Self::Output>
    where
        Self: Sized + 'static,
    {
        Boxed::new(self)
    }

    fn map<F, U>(self, transform: F) -> Map<Self, F, Self::Output>
    where
        Self: Sized,
        F: Fn(Self::Output) -> U,
    {
        Map::new(self, transform)
    }

    fn to<U: Clone>(self, value: U) -> To<Self, U>
    where
        Self: Sized,
    {
        To::new(self, value)
    }

    fn filter<F>(self, filter: F) -> Filter<Self, F>
    where
        Self: Sized,
        F: Fn(&Self::Output) -> bool,
    {
        Filter::new(self, filter)
    }

    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional::new(self)
    }

    fn then<P>(self, next: P) -> Then<Self, P>
    where
        Self: Sized,
        P: Parser,
    {
        Then::new(self, false, next, false)
    }

    fn ignore_then<P>(self, next: P) -> IgnoreThen<Self, P>
    where
        Self: Sized,
        P: Parser,
    {
        IgnoreThen::new(self, next)
    }

    fn then_ignore<P>(self, next: P) -> ThenIgnore<Self, P>
    where
        Self: Sized,
        P: Parser,
    {
        ThenIgnore::new(self, next, false)
    }

    fn then_expect_ignore<P>(self, next: P) -> ThenIgnore<Self, P>
    where
        Self: Sized,
        P: Parser,
    {
        ThenIgnore::new(self, next, true)
    }

    fn repeated(self) -> Repeated<Self>
    where
        Self: Sized,
    {
        Repeated::new(self)
    }

    fn repeated_until<T>(self, terminator: T) -> RepeatedUntil<Self, T>
    where
        Self: Sized,
        T: Parser,
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
