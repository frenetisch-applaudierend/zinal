use std::borrow::Cow;

use proc_macro2::Span;

use self::input::Input;

mod common;
mod html;
mod input;

pub fn parse<'src>(
    source: &'src str,
    content_type: &str,
) -> Result<(Vec<Item<'src>>, syn::TypePath), syn::Error> {
    let input = Input::new(source);
    let (mut parser, content_type_ty) = read_content_type(content_type)?;

    parser.parse(input).map(|items| (items, content_type_ty))
}

fn read_content_type(
    content_type: &str,
) -> Result<(impl TemplateParser, syn::TypePath), syn::Error> {
    match content_type {
        "html" => Ok((
            html::HtmlParser,
            parse_quote!(::stardust::content_types::Html),
        )),
        "plain" | "txt" => Ok((
            html::HtmlParser,
            parse_quote!(::stardust::content_types::Html),
        )),

        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "unsupported content type",
            ))
        }
    }
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
        match self {
            Keyword::If
            | Keyword::Else
            | Keyword::ElseIf
            | Keyword::For
            | Keyword::While
            | Keyword::Loop => true,

            _ => false,
        }
    }

    pub fn is_block_terminator(self) -> bool {
        match self {
            Keyword::Else | Keyword::ElseIf | Keyword::End => true,

            _ => false,
        }
    }
}
