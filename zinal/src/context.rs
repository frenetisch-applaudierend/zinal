use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
};

use crate::{Children, HtmlEscaper, Renderable, Template};

/// Context provided to templates during rendering.
///
/// Most importantly, the context provides methods to render
/// various elements of templates.
pub struct RenderContext<'a> {
    writer: &'a mut dyn std::fmt::Write,
    escaper: HtmlEscaper,
    params: TypeMap,
}

impl<'a> RenderContext<'a> {
    /// Creates a new [`RenderContext`] from the given writer.
    pub fn new(writer: &'a mut dyn std::fmt::Write) -> Self {
        Self {
            writer,
            escaper: HtmlEscaper,
            params: TypeMap::new(),
        }
    }

    /// Render a literal string to the output of this template.
    ///
    /// The content will not be escaped and is included in the rendered output verbatim.
    ///
    /// # Errors
    ///
    /// This function will return an error if the underlying writer returns an error.
    pub fn render_literal(&mut self, literal: &str) -> Result<(), std::fmt::Error> {
        write!(self.writer, "{}", literal)
    }

    /// Render a child template in the output of this template.
    ///
    /// # Errors
    ///
    /// This function will return an error if the child template rendering raises an error.
    pub fn render_template(&mut self, template: impl Template) -> Result<(), std::fmt::Error> {
        template.render(self)
    }

    /// Render an expression to the output of this template.
    ///
    /// Expressions can either be a [`Renderable`] impl or a [`Children`] value.
    /// [`Renderable`] values are HTML escaped if applicable.
    ///
    /// NOTE: This method is intended for use in the derive macro. If you have
    ///       a concrete `Renderable` value you can use [`render_renderable`] instead,
    ///       or if you have a `Children` value you can use [`render_children`] directly.
    ///
    /// # Errors
    ///
    /// This function will return an error if the expression raises an error during rendering.
    pub fn render_expression<'b>(
        &'b mut self,
        expression: impl Into<RenderExpression<'b>>,
    ) -> Result<(), std::fmt::Error> {
        match expression.into() {
            RenderExpression::Renderable(renderable) => self.render_renderable(renderable),
            RenderExpression::Children(children) => self.render_children(children),
        }
    }

    /// Render a [`Renderable`] impl to the output of this template.
    ///
    /// # Errors
    ///
    /// This function will return an error if the renderable value raises an error during rendering.
    pub fn render_renderable(
        &mut self,
        renderable: impl Renderable,
    ) -> Result<(), std::fmt::Error> {
        renderable.render_to(self.writer, &self.escaper)
    }

    /// Render a [`Children`] value to the output of this template.
    ///
    /// # Errors
    ///
    /// This function will return an error if the children value raises an error during rendering.
    pub fn render_children(&mut self, children: &Children) -> Result<(), std::fmt::Error> {
        children.render(self)
    }

    /// Sets a context wide parameter of type T.
    pub fn provide_param<P: Any>(&mut self, value: Ctx<P>) {
        let type_id = TypeId::of::<P>();
        self.params.insert(type_id, value);
    }

    /// Returns a context wide parameter of type T if it was set before.
    pub fn get_param<P: Any>(&self) -> Option<Ctx<P>> {
        let type_id = TypeId::of::<P>();
        self.params
            .get(&type_id)
            .cloned()
            .map(|p| p.downcast().expect("type was checked by TypeId"))
    }
}

/// Wrapper for values provided by the RenderContext.
pub type Ctx<T> = Rc<T>;

/// Types that can be rendered as an expression.
///
/// Encapsulates either [`Renderable`] impls or [`Children`] values.
pub enum RenderExpression<'a> {
    /// A [`Renderable`] expression.
    Renderable(&'a dyn Renderable),

    /// A [`Children`] expression.
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

type TypeMap = HashMap<TypeId, Ctx<dyn Any>>;
