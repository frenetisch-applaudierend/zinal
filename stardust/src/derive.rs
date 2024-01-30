pub enum Property<T> {
    Unset,
    Default,
    Set(T),
}

pub trait HasProperty<Prop, Tail> {}

pub struct WithProperty<Prop, Tail>(Prop, Tail);

impl<Prop, Tail> WithProperty<Prop, Tail> {
    pub fn new(prop: Prop, tail: Tail) -> Self {
        Self(prop, tail)
    }
}

pub struct Directly;

pub struct Step<N>(N);

impl<Prop, Tail> HasProperty<Prop, Directly> for WithProperty<Prop, Tail> {}

impl<Prop, AnyProp, Tail, N> HasProperty<Prop, Step<N>> for WithProperty<AnyProp, Tail> where
    Tail: HasProperty<Prop, N>
{
}
