pub enum Property<T> {
    Unset,
    Default,
    Set(T),
}
