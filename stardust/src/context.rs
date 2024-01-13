use crate::{Children, HtmlEscaper, Renderable, Template};

pub struct RenderContext<'a> {
    writer: &'a mut dyn std::fmt::Write,
    escaper: HtmlEscaper,
}

impl<'a> RenderContext<'a> {
    pub fn new(writer: &'a mut dyn std::fmt::Write) -> Self {
        Self {
            writer,
            escaper: HtmlEscaper,
        }
    }

    pub fn render_literal(&mut self, literal: &str) -> Result<(), std::fmt::Error> {
        write!(self.writer, "{}", literal)
    }

    pub fn render_template(&mut self, template: impl Template) -> Result<(), std::fmt::Error> {
        template.render(self)
    }

    pub fn render_expression<'b>(
        &'b mut self,
        expression: impl Into<RenderExpression<'b>>,
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

    pub fn render_children(&mut self, children: &Children) -> Result<(), std::fmt::Error> {
        children.render(self)
    }
}

pub enum RenderExpression<'a> {
    Renderable(&'a dyn Renderable),
    Children(&'a Children<'a>),
}

impl<'a, T> From<&'a T> for RenderExpression<'a>
where
    T: Renderable,
{
    fn from(value: &'a T) -> Self {
        Self::Renderable(value)
    }
}

impl<'a> From<&'a Children<'a>> for RenderExpression<'a> {
    fn from(value: &'a Children) -> Self {
        Self::Children(value)
    }
}
