//! Utilities for rendering HTML.

use std::{borrow::Cow, collections::HashMap, sync::OnceLock};

use crate::{Escaper, Renderable};

/// An escaper that escapes strings to be safely included in HTML content.
pub struct HtmlEscaper;

impl Escaper for HtmlEscaper {
    /// Escape the given value to be HTML safe.
    ///
    /// If the value is already safe, it is returned unchanged.
    fn escape<'a>(&self, value: Cow<'a, str>) -> Cow<'a, str> {
        let mut escaped = String::new();
        let escapes = ESCAPES.get_or_init(|| {
            HashMap::from([
                ('<', "&lt;"),
                ('>', "&gt;"),
                ('&', "&amp;"),
                ('\'', "&apos;"),
                ('"', "&quot;"),
            ])
        });

        let mut previous_offset = 0;
        let mut offset = 0;

        for c in value.chars() {
            if let Some(replacement) = escapes.get(&c) {
                escaped.push_str(&value[previous_offset..offset]);
                escaped.push_str(replacement);

                offset += c.len_utf8();
                previous_offset = offset;
            } else {
                offset += c.len_utf8();
            }
        }

        if !escaped.is_empty() {
            escaped.push_str(&value[previous_offset..offset]);
            escaped.into()
        } else {
            value
        }
    }
}

static ESCAPES: OnceLock<HashMap<char, &'static str>> = OnceLock::new();

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
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
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
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error>;
}

impl AttributeValue for &str {
    fn render_attribute(
        &self,
        name: &str,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
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
        escaper: &dyn Escaper,
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
        _escaper: &dyn Escaper,
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
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error> {
        match self {
            Some(value) => value.render_attribute(name, writer, escaper),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::Escaper as _;

    use super::HtmlEscaper;

    #[test]
    fn escaper_unescaped() {
        let input = Cow::from("This does not need to be escaped");

        let output = HtmlEscaper.escape(input);

        assert_eq!(
            output,
            Cow::<str>::Borrowed("This does not need to be escaped")
        );
    }

    #[test]
    fn escaper_escaped() {
        let input = Cow::from("<&'\">");

        let output = HtmlEscaper.escape(input);

        assert_eq!(
            output,
            Cow::<str>::Owned(String::from("&lt;&amp;&apos;&quot;&gt;"))
        );
    }

    #[test]
    fn escaper_mixed() {
        let input = Cow::from("< 'hello' & \"world\" >");

        let output = HtmlEscaper.escape(input);

        assert_eq!(
            output,
            Cow::<str>::Owned(String::from(
                "&lt; &apos;hello&apos; &amp; &quot;world&quot; &gt;"
            ))
        );
    }
}
