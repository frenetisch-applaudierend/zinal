use crate::Parser;

pub struct Boxed<O> {
    inner: Box<dyn Parser<O>>,
}

impl<'src, O> Boxed<O> {
    pub fn new(parser: impl Parser<O> + 'static) -> Self {
        Self {
            inner: Box::new(parser),
        }
    }
}

impl<O> Parser<O> for Boxed<O> {
    fn parse<'src>(&self, input: &mut crate::Input<'src>) -> crate::ParseResult<O> {
        self.inner.parse(input)
    }
}
