use crate::Parser;

pub struct Boxed<'src, T> {
    inner: Box<dyn Parser<'src, Output = T>>,
}
