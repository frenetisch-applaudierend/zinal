use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// Context parameters for templates.
pub struct Context<'a> {
    parent: Option<&'a Context<'a>>,
    params: TypeMap,
}

impl<'a> Context<'a> {
    /// Creates a new [`ContextParams`].
    pub fn new() -> Self {
        Self {
            parent: None,
            params: TypeMap::new(),
        }
    }

    /// Returns a context wide parameter of type T if it was set before.
    pub fn get_param<P: Any>(&self) -> Option<&P> {
        let type_id = TypeId::of::<P>();
        self.params
            .get(&type_id)
            .map(|p| p.downcast_ref().expect("type was checked by TypeId"))
            .or_else(|| self.parent.and_then(|p| p.get_param()))
    }

    /// Sets a context wide parameter of type T.
    pub fn provide_param<P: Any + 'static>(&mut self, value: P) {
        let type_id = TypeId::of::<P>();
        let value = Box::new(value);
        self.params.insert(type_id, value);
    }

    /// Extend this context with a child context.
    pub fn extend(&'a self, context: Context) -> Self {
        Self {
            parent: Some(self),
            params: context.params,
        }
    }
}

impl Default for Context<'_> {
    fn default() -> Self {
        Self::new()
    }
}

type TypeMap = HashMap<TypeId, Box<dyn Any>>;
