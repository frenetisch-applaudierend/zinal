use crate::{Input, Offset, ParseResult, Parser};

pub fn whitespace<'src>() -> Whitespace {
    Whitespace { min_count: 0 }
}

pub struct Whitespace {
    min_count: usize,
}

impl Whitespace {
    pub fn not_empty(self) -> Self {
        Self { min_count: 1 }
    }
}

impl Parser for Whitespace {
    type Output<'src> = Offset<'src>;

    fn parse<'src>(&self, input: &mut Input<'src>) -> ParseResult<Self::Output> {
        let position = input.position();
        let consumed = input.consume_while(char::is_whitespace);

        if consumed.len() < self.min_count {
            input.reset_to(position);
            Ok(None)
        } else {
            Ok(Some(consumed))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{whitespace, Input, Offset, Parser};

    #[test]
    fn test_empty_whitespace() {
        let mut input = Input::new("Foo");
        let parser = whitespace();

        let result = parser.parse(&mut input);

        assert_eq!(Ok(Some(Offset::new("", 0))), result);
    }
}
