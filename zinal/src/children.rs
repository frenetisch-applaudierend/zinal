use crate::RenderContext;

pub struct Children<'a> {
    renderer: &'a dyn Fn(&mut RenderContext) -> Result<(), std::fmt::Error>,
}

impl<'a> Children<'a> {
    pub fn new<F: Fn(&mut RenderContext) -> Result<(), std::fmt::Error>>(renderer: &'a F) -> Self {
        Self { renderer }
    }
    pub fn render(&self, context: &mut RenderContext) -> Result<(), std::fmt::Error> {
        (self.renderer)(context)
    }
}
