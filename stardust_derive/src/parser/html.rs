use std::borrow::Cow;

use crate::parser::common::literal;

use super::{
    common::{Combinator, ParseResult},
    input::Input,
    Error, Item, TemplateParser,
};

pub struct HtmlParser;

impl TemplateParser for HtmlParser {
    fn parse<'src>(&mut self, mut input: Input<'src>) -> Result<Vec<Item<'src>>, Error> {
        let mut items = Vec::<Item<'src>>::new();
        while !input.is_at_end() {
            match parse_item(&mut input)? {
                Some(item) => items.push(item),
                None => return Err(Error::new("Unrecognized content")),
            }
        }
        Ok(items)
    }
}

fn parse_item<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    select! {
        parse_escape,
        parse_expr,
        parse_statement,
        parse_component,
        parse_literal
    }
    .parse(input)
}

fn parse_escape<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    select! {
        literal("{{").map(|_| Cow::from("{")),
        literal("<##").map(|_| Cow::from("<#"))
    }
    .map(Item::Literal)
    .parse(input)
}

fn parse_expr<'src>(_input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    todo!()
}

fn parse_statement<'src>(_input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    todo!()
}

fn parse_component<'src>(_input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    todo!()
}

fn parse_literal<'src>(_input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::parser::{input::Input, Item, TemplateParser};

    use super::HtmlParser;

    #[test]
    fn top_level_escapes() {
        let mut parser = HtmlParser;

        let input = Input::new("{{<##");
        let result = parser.parse(input);

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(
            result.unwrap(),
            vec![
                Item::Literal(Cow::from("{")),
                Item::Literal(Cow::from("<#"))
            ]
        );
    }

    #[test]
    fn literal_plain() {
        let mut parser = HtmlParser;

        let input = Input::new("Hello, World!");
        let result = parser.parse(input);

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(
            result.unwrap(),
            vec![Item::Literal(Cow::from("Hello, World!"))]
        );
    }

    #[test]
    fn expression() {
        let mut parser = HtmlParser;

        let input = Input::new("{self.name.to_ascii_uppercase()}");
        let result = parser.parse(input);

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(
            result.unwrap(),
            vec![Item::Expression(Cow::from(
                "self.name.to_ascii_uppercase()"
            ))]
        );
    }
}
