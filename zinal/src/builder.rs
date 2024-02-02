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

pub struct TemplateBuilder<Values, Token> {
    pub values: Values,
    _token: PhantomData<Token>,
}

impl<Values, Token> TemplateBuilder<Values, Token> {
    pub fn new(values: Values) -> Self {
        Self {
            values,
            _token: PhantomData,
        }
    }

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

pub trait HasProperty<Prop, Tail> {}

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
