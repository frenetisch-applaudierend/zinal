use std::borrow::Cow;

use crate::content_type::Escaper;

pub trait Renderable {
    fn render_to(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error>;
}

impl<T> Renderable for T
where
    T: std::fmt::Display,
{
    fn render_to(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error> {
        let raw = format!("{}", self);
        let escaped = escaper.escape_string(Cow::Owned(raw));

        write!(writer, "{}", escaped)
    }
}

impl Renderable for &dyn Renderable {
    fn render_to(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error> {
        Renderable::render_to(*self, writer, escaper)
    }
}
