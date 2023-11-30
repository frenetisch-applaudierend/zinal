use std::ops::{Bound, RangeBounds};

pub struct Input<'src> {
    source: &'src str,
}

impl<'src> Input<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source }
    }

    pub fn is_at_end(&self) -> bool {
        self.source.is_empty()
    }

    pub fn consume_lit(&mut self, value: &str) -> Option<&'src str> {
        if self.source.starts_with(value) {
            let (consumed, remainder) = self.source.split_at(value.len());
            self.source = remainder;
            Some(consumed)
        } else {
            None
        }
    }

    pub fn consume_until(&mut self, value: &str) -> Option<&'src str> {
        let index = self.source.find(value)?;

        let (consumed, remainder) = self.source.split_at(index);
        self.source = remainder;
        Some(consumed)
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn try_consume() {
        let mut input = Input::new("Hellö, World!");
        assert_eq!(input.consume_lit("Hellö"), Some("Hellö"));

        let mut input = Input::new("Hellö, World!");
        assert_eq!(input.consume_lit("Hellö, World!"), Some("Hellö, World!"));

        let mut input = Input::new("Hellö, World!");
        assert_eq!(input.consume_lit("Hello"), None);
    }
}
