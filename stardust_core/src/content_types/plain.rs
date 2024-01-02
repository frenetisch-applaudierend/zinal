use crate::{ContentType, Escaper};

pub struct PlainText;

impl ContentType for PlainText {
    type Escaper = PlainTextEscaper;

    fn escaper() -> Self::Escaper {
        PlainTextEscaper
    }
}

pub struct PlainTextEscaper;

impl Escaper for PlainTextEscaper {
    fn escape_string<'a>(&self, value: std::borrow::Cow<'a, str>) -> std::borrow::Cow<'a, str> {
        value
    }
}
