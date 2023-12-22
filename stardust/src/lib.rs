use std::{
    borrow::Cow,
    fmt::{Display, Error, Write},
};

#[cfg(test)]
mod test;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use stardust_derive::Template;

pub trait Renderable {
    fn render_to(&self, writer: &mut dyn Write, escaper: &dyn Escaper) -> Result<(), Error>;
}

pub trait Template: Renderable {
    fn render_to_string(&self, escaper: &dyn Escaper) -> Result<String, Error> {
        let mut buf = String::new();
        self.render_to(&mut buf, escaper)?;
        Ok(buf)
    }
}

pub struct FuncRenderable<F>(F)
where
    F: Fn(&mut dyn Write, &dyn Escaper) -> Result<(), Error>;

impl<F> FuncRenderable<F>
where
    F: Fn(&mut dyn Write, &dyn Escaper) -> Result<(), Error>,
{
    pub fn new(func: F) -> Self {
        Self(func)
    }
}

impl<F> Renderable for FuncRenderable<F>
where
    F: Fn(&mut dyn Write, &dyn Escaper) -> Result<(), Error>,
{
    fn render_to(&self, writer: &mut dyn Write, escaper: &dyn Escaper) -> Result<(), Error> {
        (self.0)(writer, escaper)
    }
}

impl<T> Renderable for T
where
    T: Display,
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

pub trait Escaper {
    fn escape_string<'a>(&self, value: Cow<'a, str>) -> Cow<'a, str>;
}

pub struct NoopEscaper;

impl Escaper for NoopEscaper {
    fn escape_string<'a>(&self, value: Cow<'a, str>) -> Cow<'a, str> {
        value
    }
}
