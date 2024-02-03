//! Helper types to implement a Template::Builder type.
//!
//! Usually you should not need to worry about these types, and
//! instead derive the Template trait, which automatically implements
//! the builder as well.
//!
//! # Manually implementing a Template::Builder
//!
//! TBD

use std::marker::PhantomData;

/// A helper type to create template builders.
///
/// You would usually wrap the TemplateBuilder in your custom
/// builder struct and use the TemplateBuilder::set() method
/// to implement your attribute setters, automatically managing
/// the token for you.
///
/// In your build() method you can then take the values from
/// the TemplateBuilder to construct your template.
pub struct TemplateBuilder<Values, Token> {
    /// This builders current template values.
    pub values: Values,
    _token: PhantomData<Token>,
}

impl<Values, Token> TemplateBuilder<Values, Token> {
    /// Create a new token builder with the given initial values.
    pub fn new(values: Values) -> Self {
        Self {
            values,
            _token: PhantomData,
        }
    }

    /// Set a property value and mark the token as including the property.
    ///
    /// You pass this method a setter that gets invoked with a mutable reference
    /// to this builders values.
    ///
    /// Returns a new builder with the token modified to include the specified property
    /// as provided.
    pub fn set<Prop>(
        self,
        setter: impl FnOnce(&mut Values),
    ) -> TemplateBuilder<Values, WithProperty<Prop, Token>> {
        let mut values = self.values;
        setter(&mut values);
        TemplateBuilder {
            values,
            _token: PhantomData,
        }
    }
}

impl<Values, Token> Default for TemplateBuilder<Values, Token>
where
    Values: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

/// Marker trait for a token to mark a property as provided.
///
/// You do not need to implement this trait, use WithProperty<Prop, Tail>
/// as the token type instead. When using TemplateBuilder, this is managed
/// for you by the builder.
pub trait HasProperty<Prop, Tail> {}

/// Token type to mark a property as provided.
///
/// This is a zero sized type that cannot be constructed but is only intended
/// to be used as PhantomData.
///
/// Use TemplateBuilder to have it manage the token for you.
pub struct WithProperty<Prop, Tail>(PhantomData<Prop>, PhantomData<Tail>);

#[doc(hidden)]
pub struct Directly;

#[doc(hidden)]
pub struct Step<N>(N);

impl<Prop, Tail> HasProperty<Prop, Directly> for WithProperty<Prop, Tail> {}

impl<Prop, AnyProp, Tail, N> HasProperty<Prop, Step<N>> for WithProperty<AnyProp, Tail> where
    Tail: HasProperty<Prop, N>
{
}
