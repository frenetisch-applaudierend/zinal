use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// Context parameters for templates.
pub struct Context {
    params: TypeMap,
}

impl Context {
    /// Creates a new [`ContextParams`].
    pub fn new() -> Self {
        Self {
            params: TypeMap::new(),
        }
    }

    /// Returns a context wide parameter of type T if it was set before.
    pub fn get_param<P: Any>(&self) -> Option<&P> {
        let type_id = TypeId::of::<P>();
        self.params
            .get(&type_id)
            .map(|p| p.downcast_ref().expect("type was checked by TypeId"))
    }

    /// Sets a context wide parameter of type T.
    pub fn provide_param<P: Any + 'static>(&mut self, value: P) {
        let type_id = TypeId::of::<P>();
        let value = Box::new(value);
        self.params.insert(type_id, value);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

type TypeMap = HashMap<TypeId, Box<dyn Any>>;
