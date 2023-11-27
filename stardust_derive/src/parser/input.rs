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

    pub fn peek(&self, loc: impl RangeBounds<usize>) -> Option<&'src str> {
        let skip = match loc.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => *v + 1,
            Bound::Unbounded => 0,
        };

        let mut peek_start = 0;
        let mut chars = self.source.chars();
        for _ in 0..skip {
            match chars.next() {
                Some(c) => peek_start += c.len_utf8(),
                None => return None,
            }
        }

        let len = match loc.end_bound() {
            Bound::Included(v) => *v - skip + 1,
            Bound::Excluded(v) => *v - skip,
            Bound::Unbounded => return Some(&self.source[peek_start..]),
        };

        let mut peek_end = peek_start;
        for _ in 0..len {
            match chars.next() {
                Some(c) => peek_end += c.len_utf8(),
                None => return None,
            }
        }

        Some(&self.source[peek_start..peek_end])
    }

    pub fn try_consume(&mut self, value: &str) -> Option<&'src str> {
        if self.source.starts_with(value) {
            let (consumed, remainder) = self.source.split_at(value.len());
            self.source = remainder;
            Some(consumed)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn peek() {
        let input = Input::new("Hellö, World!");

        assert_eq!(input.peek(..3), Some("Hel"));
        assert_eq!(input.peek(3..), Some("lö, World!"));
        assert_eq!(input.peek(3..5), Some("lö"));
        assert_eq!(input.peek(3..=5), Some("lö,"));
        assert_eq!(input.peek(..), Some("Hellö, World!"));
    }

    #[test]
    fn try_consume() {
        let mut input = Input::new("Hellö, World!");
        assert_eq!(input.try_consume("Hellö"), Some("Hellö"));

        let mut input = Input::new("Hellö, World!");
        assert_eq!(input.try_consume("Hellö, World!"), Some("Hellö, World!"));

        let mut input = Input::new("Hellö, World!");
        assert_eq!(input.try_consume("Hello"), None);
    }
}
