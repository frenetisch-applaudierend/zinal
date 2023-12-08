mod combinators;
mod generators;
mod input;
mod parsers;

use std::borrow::Cow;

pub use combinators::*;
pub use generators::*;
pub use input::*;
pub use parsers::*;

pub trait Parser<'src>: Sized {
    type Output;

    fn parse(&self, input: &mut Input<'src>) -> ParseResult<Self::Output>;

    fn peek(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        let position = input.position();
        let result = self.parse(input);
        input.reset_to(position);
        result
    }

    fn map<F, U>(self, transform: F) -> Map<Self, F, Self::Output>
    where
        F: Fn(Self::Output) -> U,
    {
        Map::new(self, transform)
    }

    fn filter<F>(self, filter: F) -> Filter<Self, F>
    where
        F: Fn(&Self::Output) -> bool,
    {
        Filter::new(self, filter)
    }

    fn optional(self) -> Optional<Self> {
        Optional::new(self)
    }

    fn then<P>(self, next: P) -> Then<Self, P>
    where
        P: Parser<'src>,
    {
        Then::new(self, next)
    }

    fn ignore_then<P>(self, next: P) -> IgnoreThen<Self, P>
    where
        P: Parser<'src>,
    {
        IgnoreThen::new(self, next)
    }

    fn then_ignore<P>(self, next: P) -> ThenIgnore<Self, P>
    where
        P: Parser<'src>,
    {
        ThenIgnore::new(self, next)
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
