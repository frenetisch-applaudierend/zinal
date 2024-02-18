//! Types used in derived Template impls. You should usually have little to no need to use these types.

use crate::{Children, Context, Escaper, Renderable};

/// Trait for items that can be rendered from expressions.
///
/// Implemented for [`Renderable`] as blanket impl and for [`Children`].
pub trait RenderExpression {
    /// Render this expression to the given writer.
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
        context: &Context,
    ) -> Result<(), std::fmt::Error>;
}

impl<T> RenderExpression for T
where
    T: Renderable,
{
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
        _context: &Context,
    ) -> Result<(), std::fmt::Error> {
        Renderable::render(self, writer, escaper)
    }
}

impl RenderExpression for Children<'_> {
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
        context: &Context,
    ) -> Result<(), std::fmt::Error> {
        Children::render(self, writer, escaper, context)
    }
}
