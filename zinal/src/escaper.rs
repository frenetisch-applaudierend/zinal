use std::borrow::Cow;

/// Trait for objects that escape some content to be safely included in their context.
pub trait Escaper {
    /// Escape the given value to be safe to be included in the escapers context.
    fn escape<'a>(&self, value: Cow<'a, str>) -> Cow<'a, str>;
}
