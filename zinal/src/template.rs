use std::any::Any;

use crate::{ParamProvider, RenderContext};

/// Trait implemented by types representing a template.
///
/// Usually you will derive this trait, as this will then parse given
/// template file or string and implement this trait accordingly.
///
/// If you need to implement this trait manually, check the zinal::builder module,
/// as well as the zinal::RenderContext type for more information.
///
/// # Examples
/// ```rust
/// # use zinal::Template;
/// #[derive(Template)]
/// #[template(content = "<div>Hello, {{self.name}}!</div>")]
/// struct Hello {
///   name: String
/// }
/// ```
///
pub trait Template: Sized {
    /// The builder type for this template.
    ///
    /// Derived templates will use this builder to render child
    /// templates. For example, given the following child template reference:
    ///
    /// ```html
    /// <FooTemplate name="John" age={{42}} />
    /// ```
    ///
    /// The derived parent template will render the child using the following
    /// method call chain:
    ///
    /// ```rust
    /// # use zinal::{Template, RenderContext};
    /// # #[derive(Template)]
    /// # #[template(content = "")]
    /// # struct FooTemplate { name: String, age: u8 }
    /// # let mut buf = String::new();
    /// # let mut context = RenderContext::new(&mut buf);
    /// FooTemplate::builder().name("John".into()).age(42).build(&mut context)
    /// # ;
    /// ```
    ///
    /// Therefore, the builder must implement the following requirements:
    /// * for each attribute the template should support, there
    ///   needs to be a setter method with the same name, and taking
    ///   a value of the attribute type.
    /// * a method named build(context: RenderContext) that creates and
    ///   returns the template with the previously set properties. The build
    ///   method should require that all required properties were previously
    ///   set.
    ///
    /// Usually this will be implemented automatically by deriving
    /// the Template trait. When implementing the trait manually,
    /// check the zinal::builder module on more information on how
    /// to implement a builder type.
    ///
    /// NOTE: The builder is intentionally not restricted to a trait, to allow
    ///       the build method to have varying generic restrictions.
    type Builder;

    /// Render this template using the given RenderContext.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the RenderContexts render_* methods fail.
    /// Usually because the underlying std::fmt::Write implementation generates an error.
    fn render(self, context: &mut RenderContext) -> Result<(), std::fmt::Error>;

    /// Render this template to a string using the render() method.
    ///
    /// # Errors
    ///
    /// This function will return an error if the render() method returns an error.
    fn render_to_string(self) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();
        let mut context = RenderContext::new(&mut buf);

        self.render(&mut context)?;

        Ok(buf)
    }

    /// Attach a parameter to the context when rendering this template.
    fn with_context_param<P: Any>(self, param: P) -> impl Template {
        ParamProvider::new(self, param)
    }

    /// Create and return a builder for this template.
    fn builder() -> Self::Builder;
}
