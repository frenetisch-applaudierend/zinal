use crate::RenderContext;

/// Represents child content to a template.
pub struct Children<'a> {
    renderer: &'a dyn Fn(&mut RenderContext) -> Result<(), std::fmt::Error>,
}

impl<'a> Children<'a> {
    /// Create a new [`Children`] value with the given renderer.
    pub fn new<F: Fn(&mut RenderContext) -> Result<(), std::fmt::Error>>(renderer: &'a F) -> Self {
        Self { renderer }
    }

    /// Render this children value to the given context.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderer returns an error.
    pub fn render(&self, context: &mut RenderContext) -> Result<(), std::fmt::Error> {
        (self.renderer)(context)
    }
}
