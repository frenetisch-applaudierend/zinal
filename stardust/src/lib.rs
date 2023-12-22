use std::fmt::{Display, Error, Write};

#[cfg(test)]
mod test;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use stardust_derive::Template;

pub trait Renderable {
    fn render_to(&self, writer: &mut dyn Write) -> Result<(), Error>;
}

pub trait Template: Renderable {
    fn render_to_string(&self) -> Result<String, Error> {
        let mut buf = String::new();
        self.render_to(&mut buf)?;
        Ok(buf)
    }
}

pub struct FuncRenderable<F>(F)
where
    F: Fn(&mut dyn Write) -> Result<(), Error>;

impl<F> FuncRenderable<F>
where
    F: Fn(&mut dyn Write) -> Result<(), Error>,
{
    pub fn new(func: F) -> Self {
        Self(func)
    }
}

impl<F> Renderable for FuncRenderable<F>
where
    F: Fn(&mut dyn Write) -> Result<(), Error>,
{
    fn render_to(&self, writer: &mut dyn Write) -> Result<(), Error> {
        (self.0)(writer)
    }
}

impl<T> Renderable for T
where
    T: Display,
{
    fn render_to(&self, writer: &mut dyn Write) -> Result<(), Error> {
        write!(writer, "{}", self)
    }
}

impl Renderable for &dyn Renderable {
    fn render_to(&self, writer: &mut dyn Write) -> Result<(), Error> {
        Renderable::render_to(*self, writer)
    }
}
