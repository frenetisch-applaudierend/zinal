use super::{Item, TemplateParser};

pub struct HtmlParser<'src> {
    source: &'src str,
}

impl<'src> HtmlParser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source }
    }

    fn try_parse_escape(&mut self) -> Option<Item<'src>> {
        None
    }

    fn try_parse_expression(&mut self) -> Option<Result<Item<'src>, super::Error>> {
        None
    }

    fn try_parse_statement(&mut self) -> Option<Result<Item<'src>, super::Error>> {
        None
    }

    fn try_parse_child_template(&mut self) -> Option<Result<Item<'src>, super::Error>> {
        None
    }

    fn parse_literal(&mut self) -> Item<'src> {
        // Skip the first match of a control character if it's the first character in the source.
        // Otherwise we'll get in an infinite loop.
        let mut skip = self.source.starts_with(['{', '<']);
        let pattern = move |c| {
            if skip {
                skip = false;
                return false;
            }

            c == '{' || c == '<'
        };

        match self.source.find(pattern) {
            Some(idx) => {
                let (literal, rest) = self.source.split_at(idx);
                self.source = rest;
                Item::Literal(literal)
            }
            None => {
                let result = Item::Literal(self.source);
                self.source = "";
                result
            }
        }
    }

    fn peek(&self, len: usize) -> &'src str {
        if self.source.len() >= len {
            &self.source[..len]
        } else {
            ""
        }
    }
}

impl<'src> TemplateParser<'src> for HtmlParser<'src> {
    fn parse_next(&mut self) -> Result<Option<Item<'src>>, super::Error> {
        if self.source.is_empty() {
            return Ok(None);
        }

        if let Some(res) = self.try_parse_escape() {
            return Ok(Some(res));
        };

        if let Some(res) = self.try_parse_expression() {
            return res.map(Some);
        };

        if let Some(res) = self.try_parse_statement() {
            return res.map(Some);
        };

        if let Some(res) = self.try_parse_child_template() {
            return res.map(Some);
        };

        Ok(Some(self.parse_literal()))
    }
}
