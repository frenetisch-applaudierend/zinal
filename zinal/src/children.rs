use crate::{Context, Escaper};

/// Represents child content to a template.
pub trait Children {
    /// Render the children to the given writer.
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
        context: &Context,
    ) -> Result<(), std::fmt::Error>;
}

impl<F> Children for F
where
    F: Fn(&mut dyn std::fmt::Write, &dyn Escaper, &Context) -> Result<(), std::fmt::Error>,
{
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
        context: &Context,
    ) -> Result<(), std::fmt::Error> {
        (self)(writer, escaper, context)
    }
}

/// Struct that represents no children for a template.
pub struct EmptyChildren;

impl Children for EmptyChildren {
    fn render(
        &self,
        _writer: &mut dyn std::fmt::Write,
        _escaper: &dyn Escaper,
        _context: &Context,
    ) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
