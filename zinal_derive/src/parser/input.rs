use std::borrow::Cow;

pub struct Input<'src> {
    source: &'src str,
    remainder: &'src str,
    offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offset<'src> {
    text: &'src str,
    offset: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position(usize);

impl<'src> Input<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            remainder: source,
            offset: 0,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.remainder.is_empty()
    }

    pub fn position(&self) -> Position {
        Position(self.offset)
    }

    pub fn reset_to(&mut self, position: Position) {
        self.offset = position.0;
        self.remainder = &self.source[self.offset..];
    }

    pub fn consume_lit(&mut self, value: &str) -> Option<Offset<'src>> {
        if self.remainder.starts_with(value) {
            Some(self.consume(value.len()))
        } else {
            None
        }
    }

    pub fn consume_while(&mut self, condition: impl Fn(char) -> bool) -> Offset<'src> {
        let mut len = 0;
        for c in self.remainder.chars() {
            if !condition(c) {
                break;
            }

            len += c.len_utf8();
        }

        self.consume(len)
    }

    pub fn consume_until(&mut self, sentinel: &str) -> Offset<'src> {
        let Some(index) = self.remainder.find(sentinel) else {
            return self.consume_all();
        };

        self.consume(index)
    }

    pub fn consume_until_any(&mut self, sentinels: &str) -> Offset<'src> {
        if self.is_at_end() {
            return Offset::new("", self.offset);
        }

        let mut len = 0;
        for c in self.remainder.chars() {
            if sentinels.contains(c) {
                break;
            }

            len += c.len_utf8();
        }

        self.consume(len)
    }

    pub fn consume_count(&mut self, count: usize) -> Option<Offset<'src>> {
        let mut taken = 0;
        let mut len = 0;
        for c in self.remainder.chars().take(count) {
            taken += 1;
            len += c.len_utf8();
        }

        if taken != count {
            return None;
        }

        Some(self.consume(len))
    }

    pub fn consume_all(&mut self) -> Offset<'src> {
        let consumed = self.remainder;
        let offset = self.offset;

        self.offset += consumed.len();
        self.remainder = "";

        Offset::new(consumed, offset)
    }

    pub fn combine(&self, values: &[Offset<'src>]) -> Offset<'src> {
        if values.is_empty() {
            panic!("values must not be empty");
        }

        let start = values[0].offset;
        let mut len = 0;

        for value in values {
            if value.offset != (start + len) {
                panic!("Values must be consecutive in the source string");
            }

            len += value.len();
        }

        Offset::new(&self.source[start..(start + len)], start)
    }

    fn consume(&mut self, len: usize) -> Offset<'src> {
        let (consumed, remainder) = self.remainder.split_at(len);
        let offset = self.offset;

        self.offset += consumed.len();
        self.remainder = remainder;

        Offset::new(consumed, offset)
    }
}

impl<'src> Offset<'src> {
    pub fn new(text: &'src str, offset: usize) -> Self {
        Self { text, offset }
    }

    pub fn into_cow(self) -> Cow<'src, str> {
        Cow::Borrowed(self.text)
    }

    pub fn into_str(self) -> &'src str {
        self.text
    }
}

impl<'src> AsRef<str> for Offset<'src> {
    fn as_ref(&self) -> &'src str {
        self.text
    }
}

impl<'src> std::ops::Deref for Offset<'src> {
    type Target = str;

    fn deref(&self) -> &'src Self::Target {
        self.text
    }
}

#[cfg(test)]
mod tests {
    use super::{Input, Offset};

    #[test]
    fn consume_lit() {
        let mut input = Input::new("Hellö, World!");
        assert_eq!(input.consume_lit("Hellö"), Some(Offset::new("Hellö", 0)));

        let mut input = Input::new("Hellö, World!");
        assert_eq!(
            input.consume_lit("Hellö, World!"),
            Some(Offset::new("Hellö, World!", 0))
        );

        let mut input = Input::new("Hellö, World!");
        assert_eq!(input.consume_lit("Hello"), None);
    }
}
