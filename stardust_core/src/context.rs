use crate::{Children, ContentType, Renderable, Template};

pub struct RenderContext<'a, C: ContentType> {
    writer: &'a mut dyn std::fmt::Write,
    escaper: C::Escaper,
}

impl<'a, C: ContentType> RenderContext<'a, C> {
    pub fn new(writer: &'a mut dyn std::fmt::Write) -> Self {
        Self {
            writer,
            escaper: C::escaper(),
        }
    }

    pub fn render_literal(&mut self, literal: &str) -> Result<(), std::fmt::Error> {
        write!(self.writer, "{}", literal)
    }

    pub fn render_template(&mut self, template: impl Template<C>) -> Result<(), std::fmt::Error> {
        template.render(self)
    }

    pub fn render_expression<'b>(
        &'b mut self,
        expression: impl Into<RenderExpression<'b, C>>,
    ) -> Result<(), std::fmt::Error> {
        match expression.into() {
            RenderExpression::Renderable(renderable) => self.render_renderable(renderable),
            RenderExpression::Children(children) => self.render_children(children),
        }
    }

    pub fn render_renderable(
        &mut self,
        renderable: impl Renderable,
    ) -> Result<(), std::fmt::Error> {
        renderable.render_to(self.writer, &self.escaper)
    }

    pub fn render_children(&mut self, children: &Children<C>) -> Result<(), std::fmt::Error> {
        children.render(self)
    }
}

pub enum RenderExpression<'a, C: ContentType> {
    Renderable(&'a dyn Renderable),
    Children(&'a Children<C>),
}

impl<'a, T, C: ContentType> From<&'a T> for RenderExpression<'a, C>
where
    T: Renderable,
{
    fn from(value: &'a T) -> Self {
        Self::Renderable(value)
    }
}

impl<'a, C: ContentType> From<&'a Children<C>> for RenderExpression<'a, C> {
    fn from(value: &'a Children<C>) -> Self {
        Self::Children(value)
    }
}
