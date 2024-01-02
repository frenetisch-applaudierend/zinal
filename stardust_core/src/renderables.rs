use std::{
    borrow::Cow,
    fmt::{Error, Write},
};

use crate::{Escaper, Renderable};

impl<T> Renderable for T
where
    T: std::fmt::Display,
{
    fn render_to(&self, writer: &mut dyn Write, escaper: &dyn Escaper) -> Result<(), Error> {
        let raw = format!("{}", self);
        let escaped = escaper.escape_string(Cow::Owned(raw));

        write!(writer, "{}", escaped)
    }
}

impl Renderable for &dyn Renderable {
    fn render_to(&self, writer: &mut dyn Write, escaper: &dyn Escaper) -> Result<(), Error> {
        Renderable::render_to(*self, writer, escaper)
    }
}
