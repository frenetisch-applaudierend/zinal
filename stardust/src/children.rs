use crate::{content_type::ContentType, RenderContext};

pub struct Children<C: ContentType> {
    #[allow(clippy::type_complexity)]
    renderer: Box<dyn Fn(&mut RenderContext<C>) -> Result<(), std::fmt::Error>>,
}

impl<C: ContentType> Children<C> {
    pub fn new<F: Fn(&mut RenderContext<C>) -> Result<(), std::fmt::Error> + 'static>(
        renderer: F,
    ) -> Self {
        Self {
            renderer: Box::new(renderer),
        }
    }
    pub fn render(&self, context: &mut RenderContext<C>) -> Result<(), std::fmt::Error> {
        (self.renderer)(context)
    }
}
