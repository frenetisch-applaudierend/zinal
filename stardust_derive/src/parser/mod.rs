use std::borrow::Cow;

use self::{html::HtmlParser, input::Input};

mod common;
mod html;
mod input;

pub fn parse<'src>(source: &'src str) -> Result<Vec<Item<'src>>, syn::Error> {
    let input = Input::new(source);
    let mut parser = HtmlParser;

    parser.parse(input).map(|items| items)
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
    ChildTemplate {
        name: Cow<'src, str>,
        arguments: Vec<TemplateArgument<'src>>,
        children: Vec<Item<'src>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    If,
    Else,
    ElseIf,
    For,
    While,
    Loop,
    End,
    Break,
    Continue,
    Let,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateArgument<'src> {
    pub(crate) name: Cow<'src, str>,
    pub(crate) value: TemplateArgumentValue<'src>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateArgumentValue<'src> {
    Literal(Cow<'src, str>),
    Expression(Cow<'src, str>),
}

impl Keyword {
    pub fn has_body(self) -> bool {
        matches!(
            self,
            Keyword::If
                | Keyword::Else
                | Keyword::ElseIf
                | Keyword::For
                | Keyword::While
                | Keyword::Loop
        )
    }

    pub fn is_block_terminator(self) -> bool {
        matches!(self, Keyword::Else | Keyword::ElseIf | Keyword::End)
    }
}
