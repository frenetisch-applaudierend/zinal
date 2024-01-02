pub mod content_types;

mod context;
mod renderables;

pub use context::*;
pub use renderables::*;

pub trait Template<C: ContentType> {
    fn render(&self, context: &mut RenderContext<C>) -> Result<(), std::fmt::Error>;

    fn render_to_string(&self) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();
        let mut context = RenderContext::new(&mut buf);

        self.render(&mut context)?;

        Ok(buf)
    }
}

pub trait ContentType {
    type Escaper: self::Escaper;

    fn escaper() -> Self::Escaper;
}

pub trait Escaper {
    fn escape_string<'a>(&self, value: std::borrow::Cow<'a, str>) -> std::borrow::Cow<'a, str>;
}

pub trait Renderable {
    fn render_to(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error>;
}

pub struct Children<C: ContentType> {
    renderer: Box<RenderFn<C>>,
}

type RenderFn<C> = dyn Fn(&mut RenderContext<C>) -> Result<(), std::fmt::Error>;

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
