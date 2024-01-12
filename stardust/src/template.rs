use crate::{content_type::ContentType, RenderContext};

pub trait Template<C: ContentType> {
    fn render(&self, context: &mut RenderContext<C>) -> Result<(), std::fmt::Error>;

    fn render_to_string(&self) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();
        let mut context = RenderContext::new(&mut buf);

        self.render(&mut context)?;

        Ok(buf)
    }
}
