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

impl<T> Renderable for T
where
    T: Display,
{
    fn render_to(&self, writer: &mut dyn Write) -> Result<(), Error> {
        write!(writer, "{}", self)
    }
}
