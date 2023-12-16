use crate::Parser;

pub struct Boxed<T> {
    inner: Box<dyn Parser<Output = T>>,
}

impl<'src, T> Boxed<T> {
    pub fn new(parser: impl Parser<Output = T> + 'static) -> Self {
        Self {
            inner: Box::new(parser),
        }
    }
}

impl<T> Parser for Boxed<T> {
    type Output = T;

    fn parse<'src>(&self, input: &mut crate::Input<'src>) -> crate::ParseResult<Self::Output> {
        self.inner.parse(input)
    }
}
