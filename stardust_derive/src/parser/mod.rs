use std::borrow::Cow;

use proc_macro2::Span;

use self::{error::Error, input::Input};

mod combinators;
mod error;
mod html;
mod input;

pub fn parse<'src>(source: &'src str, content_type: &str) -> Result<Vec<Item<'src>>, syn::Error> {
    let input = Input::new(source);
    let mut parser = match content_type {
        "html" => html::HtmlParser,
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "unsupported content type",
            ))
        }
    };

    parser
        .parse(input)
        .map_err(|err| syn::Error::new(Span::call_site(), err.message))
}

pub trait TemplateParser {
    fn parse<'src>(&mut self, input: Input<'src>) -> Result<Vec<Item<'src>>, Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item<'src> {
    Literal(Cow<'src, str>),
    Expression(Cow<'src, str>),
    KeywordStatement {
        keyword: Keyword,
        statement: Option<Cow<'src, str>>,
        body: Vec<Item<'src>>,
    },
    PlainStatement(Cow<'src, str>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    If,
    Else,
    ElseIf,
    For,
    While,
    Loop,
    Break,
    Continue,
    Let,
}

impl Keyword {
    pub fn requires_body(self) -> bool {
        match self {
            Keyword::If => true,
            Keyword::Else => true,
            Keyword::ElseIf => true,
            Keyword::For => true,
            Keyword::While => true,
            Keyword::Loop => true,

            Keyword::Break => false,
            Keyword::Continue => false,
            Keyword::Let => false,
        }
    }
}
