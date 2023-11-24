use std::fmt::Display;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;

pub mod common;
pub mod html;

pub fn parse<'src>(source: &'src str, content_type: &str) -> Result<Vec<Item<'src>>, syn::Error> {
    let mut parser = match content_type {
        "html" => html::HtmlParser::new(),
        _ => return Err(syn::Error::new_spanned("unsupported content type", source)),
    };

    let mut items = Vec::new();
    while let Some(item) = parser
        .parse(source)
        .map_err(|err| syn::Error::new(Span::call_site(), err.message))?
    {
        items.push(item);
    }
    Ok(items)
}

pub trait TemplateParser {
    fn parse<'src>(&mut self, source: &'src str) -> Result<Vec<Item<'src>>, Error>;
}

#[derive(Debug)]
pub enum Item<'src> {
    Literal(&'src str),
    Expression(&'src str),
    BlockStatement {
        keyword: BlockKeyword,
        expr: &'src str,
        body: Vec<Item<'src>>,
    },
    KeywordStatement {
        keyword: InlineKeyword,
        statement: Option<&'src str>,
    },
    PlainStatement(&'src str),
}

#[derive(Debug)]
pub enum BlockKeyword {
    If,
    Else,
    For,
    While,
    Loop,
}

#[derive(Debug)]
pub enum InlineKeyword {
    Break,
    Continue,
    Let,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn premature_eof() -> Self {
        Self::new("premature end of file")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for Error {}
