mod html;
mod plain;

pub use html::*;
pub use plain::*;

pub trait ContentType {
    type Escaper: self::Escaper;

    fn escaper() -> Self::Escaper;
}

pub trait Escaper {
    fn escape_string<'a>(&self, value: std::borrow::Cow<'a, str>) -> std::borrow::Cow<'a, str>;
}
