use super::{Item, TemplateParser};

pub struct HtmlParser;

impl TemplateParser for HtmlParser {
    fn parse<'src>(&mut self, source: &'src str) -> Result<Vec<Item<'src>>, super::Error> {
        todo!()
    }
}
