//! Utilities for rendering HTML.

use std::borrow::Cow;

use crate::{HtmlEscaper, Renderable};

/// Render an attribute with the given value.
///
/// # Example
/// ```rust
/// use zinal::*;
///
/// #[derive(Template)]
/// #[template("<input {{html::attr(\"readonly\", self.read_only)}}>")]
/// struct Input {
///   read_only: bool
/// }
///
/// let input1 = Input { read_only: true };
/// let input2 = Input { read_only: false };
///
/// assert_eq!(Ok("<input readonly>".to_owned()), input1.render_to_string());
/// assert_eq!(Ok("<input >".to_owned()), input2.render_to_string());
/// ```
pub fn attr<T: AttributeValue>(name: &str, value: T) -> Attribute<'_, T> {
    Attribute { name, value }
}

/// A Renderable that represents a HTML attribute.
pub struct Attribute<'a, T: AttributeValue> {
    /// The name of this attribute.
    pub name: &'a str,
    /// The value of this attribute.
    pub value: T,
}

impl<'a, T: AttributeValue> Renderable for Attribute<'a, T> {
    fn render_to(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &crate::HtmlEscaper,
    ) -> Result<(), std::fmt::Error> {
        self.value.render_attribute(self.name, writer, escaper)
    }
}

/// Trait representing valid values of a HTML attribute.
///
/// Implemented for &str, String and bool, as well as Optional<T> where T: AttributeValue.
pub trait AttributeValue {
    /// Render the attribute for this value with the given name.
    ///
    /// # Errors
    ///
    /// This function will return an error if the writer returns an error when writing.
    fn render_attribute(
        &self,
        name: &str,
        writer: &mut dyn std::fmt::Write,
        escaper: &HtmlEscaper,
    ) -> Result<(), std::fmt::Error>;
}

impl AttributeValue for &str {
    fn render_attribute(
        &self,
        name: &str,
        writer: &mut dyn std::fmt::Write,
        escaper: &HtmlEscaper,
    ) -> Result<(), std::fmt::Error> {
        write!(
            writer,
            "{}=\"{}\"",
            name,
            escaper.escape(Cow::Borrowed(self))
        )
    }
}

impl AttributeValue for String {
    fn render_attribute(
        &self,
        name: &str,
        writer: &mut dyn std::fmt::Write,
        escaper: &HtmlEscaper,
    ) -> Result<(), std::fmt::Error> {
        write!(
            writer,
            "{}=\"{}\"",
            name,
            escaper.escape(Cow::Borrowed(self))
        )
    }
}

impl AttributeValue for bool {
    fn render_attribute(
        &self,
        name: &str,
        writer: &mut dyn std::fmt::Write,
        _escaper: &HtmlEscaper,
    ) -> Result<(), std::fmt::Error> {
        if !(*self) {
            return Ok(());
        }
        write!(writer, "{}", name,)
    }
}

impl<T> AttributeValue for Option<T>
where
    T: AttributeValue,
{
    fn render_attribute(
        &self,
        name: &str,
        writer: &mut dyn std::fmt::Write,
        escaper: &HtmlEscaper,
    ) -> Result<(), std::fmt::Error> {
        match self {
            Some(value) => value.render_attribute(name, writer, escaper),
            None => Ok(()),
        }
    }
}
