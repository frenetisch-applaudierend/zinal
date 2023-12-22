use std::borrow::Cow;

use proc_macro2::Span;

use self::input::Input;

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

    parser.parse(input)
}

pub trait TemplateParser {
    fn parse<'src>(&mut self, input: Input<'src>) -> Result<Vec<Item<'src>>, syn::Error>;
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
    pub(crate) name: &'src str,
    pub(crate) value: TemplateArgumentValue<'src>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateArgumentValue<'src> {
    Literal(Cow<'src, str>),
    Expression(Cow<'src, str>),
}

impl Keyword {
    pub fn has_body(self) -> bool {
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

            Keyword::End => unreachable!(),
        }
    }
}
