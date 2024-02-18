use crate::{Context, Escaper};

/// Represents child content to a template.
pub struct Children<'a> {
    renderer: &'a RenderFn,
}

/// Function type to render content.
pub type RenderFn =
    dyn Fn(&mut dyn std::fmt::Write, &dyn Escaper, &Context) -> Result<(), std::fmt::Error>;

impl<'a> Children<'a> {
    /// Create a new [`Children`] value with the given renderer.
    pub fn new(renderer: &'a RenderFn) -> Self {
        Self { renderer }
    }

    /// Render this children value to the given context.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderer returns an error.
    pub fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
        context: &Context,
    ) -> Result<(), std::fmt::Error> {
        (self.renderer)(writer, escaper, context)
    }
}
