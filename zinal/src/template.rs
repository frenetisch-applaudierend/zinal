use crate::RenderContext;

pub trait Template: Sized {
    type Builder;

    fn render(&self, context: &mut RenderContext) -> Result<(), std::fmt::Error>;

    fn render_to_string(&self) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();
        let mut context = RenderContext::new(&mut buf);

        self.render(&mut context)?;

        Ok(buf)
    }

    fn builder() -> Self::Builder;
}
