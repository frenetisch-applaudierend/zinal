use std::fmt::Display;

pub trait TemplateParser<'src> {
    fn parse_next(&mut self) -> Result<Option<Item<'src>>, Error>;
}

#[derive(Debug)]
pub enum Item<'src> {
    Literal(&'src str),
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for Error {}

pub fn parse<'src>(source: &'src str, _content_type: &str) -> Result<Vec<Item<'src>>, syn::Error> {
    Ok(vec![Item::Literal(source)])
}
