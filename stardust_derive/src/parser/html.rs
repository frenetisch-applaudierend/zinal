use super::{common::Source, Item, TemplateParser};

pub struct HtmlParser<'src> {
    source: Source<'src>,
}

impl<'src> HtmlParser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source: Source::new(source),
        }
    }

    fn try_parse_escape(&mut self) -> Option<Item<'src>> {
        if self.source.try_consume("{{") {
            return Some(Item::Literal("{"));
        }

        if self.source.try_consume("<%%") {
            return Some(Item::Literal("<%"));
        }

        None
    }

    fn try_parse_expression(&mut self) -> Option<Result<Item<'src>, super::Error>> {
        if !self.source.try_consume("{") {
            return None;
        }

        Some(self.source.parse_rust_expr("}", "}}").map(Item::Expression))
    }

    fn try_parse_statement(&mut self) -> Option<Result<Item<'src>, super::Error>> {
        if !self.source.try_consume("<%") {
            return None;
        }

        Some(
            self.source
                .parse_rust_statement("%>", "%%>")
                .map(Item::Statement),
        )
    }

    fn try_parse_child_template(&mut self) -> Option<Result<Item<'src>, super::Error>> {
        None
    }

    fn parse_literal(&mut self) -> Item<'src> {
        Item::Literal(self.source.consume_until(&['{', '<'], true))
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
