use std::borrow::Cow;

use crate::{Input, ParseResult, Parser};

pub fn take_until<'src, T>(terminator: T) -> TakeUntil<T>
where
    T: Parser<'src>,
{
    TakeUntil {
        terminator,
        escape: None,
    }
}

#[derive(Debug, Clone)]
pub struct TakeUntil<T> {
    terminator: T,
    escape: Option<(&'static str, &'static str)>,
}

impl<T> TakeUntil<T> {
    pub fn escape(self, escape: &'static str, replacement: &'static str) -> TakeUntil<T> {
        Self {
            terminator: self.terminator,
            escape: Some((escape, replacement)),
        }
    }
}

impl<'src, T> Parser<'src> for TakeUntil<T>
where
    T: Parser<'src>,
{
    type Output = Cow<'src, str>;

    fn parse(&self, input: &mut Input<'src>) -> ParseResult<Cow<'src, str>> {
        let mut result = Cow::from("");
        let mut start = input.position();

        loop {
            // Handle escape sequence
            if let Some((escape, replacement)) = self.escape {
                let end = input.position();
                if input.consume_lit(escape).is_some() {
                    // Push still open inputs first. We can call to_mut() unconditionally
                    // as we need to change to owned for the replacement anyway
                    result.to_mut().push_str(&input.peek_range(start..end));
                    start = input.position();

                    result.to_mut().push_str(replacement);

                    continue;
                }
            }

            // Handle terminator
            if self.terminator.peek(input)?.is_some() || !input.advance() {
                // Push still open input first. Take care not to convert the result to owned
                // if not needed
                let end = input.position();
                let value = input.peek_range(start..end);

                if result.is_empty() {
                    result = value.into_cow();
                } else {
                    result.to_mut().push_str(value.as_ref());
                }

                return Ok(Some(result));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{literal, take_until, whitespace, Input, Parser};

    #[test]
    fn literal_with_whitespace() {
        let mut input = Input::new("Foobar baz   <EOF>");

        let parser = take_until(whitespace().then(literal("<EOF>")));

        let result = parser.parse(&mut input);

        assert_eq!(Ok(Some(Cow::from("Foobar baz"))), result);
    }

    #[test]
    fn escapes() {
        let mut input = Input::new("Foobar baz <<EOF>><EOF>");

        let parser = take_until(literal("<EOF>")).escape("<<EOF>>", "<EOF>");

        let result = parser.parse(&mut input);

        assert_eq!(Ok(Some(Cow::Owned("Foobar baz <EOF>".to_string()))), result);
    }
}
