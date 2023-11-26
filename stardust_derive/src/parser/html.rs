use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_till1, take_until1, take_while1},
        streaming::take_until,
    },
    character::complete::char,
    combinator::eof,
    error::ParseError,
    multi::{fold_many0, many0},
    sequence::delimited,
    IResult, Parser,
};

use super::{Error, Item, TemplateParser};

pub struct HtmlParser;

impl HtmlParser {
    fn parse_item<'src>(input: &'src str) -> IResult<&'src str, Item<'src>> {
        alt((Self::parse_expr, Self::parse_literal)).parse(input)
    }

    fn parse_expr<'src>(input: &'src str) -> IResult<&'src str, Item<'src>> {
        let content_lit = take_till1(|c| c == '}');
        let content_esc = tag("}}");
        let content_frag = alt((content_lit, content_esc));
        let content = fold_many0(
            content_frag,
            Cow::default,
            |mut acc: Cow<'src, str>, item: &str| {
                if acc.as_ref().is_empty() {
                    Cow::from(item)
                } else {
                    acc.to_mut().push_str(item);
                    acc
                }
            },
        );

        delimited(char('{'), content, char('}'))
            .map(Item::Expression)
            .parse(input)
    }

    fn parse_literal<'src>(input: &'src str) -> IResult<&str, Item<'src>> {
        let plain = take_till1(|c| c == '{' || c == '<').map(|s| Item::Literal(s));
        let escape = alt((
            tag("{{").map(|_| Item::Literal("{")),
            tag("<##").map(|_| Item::Literal("<#")),
        ));
        let tag = ??;

        alt((plain, escape))(input)
    }

    fn combine_cow<'src, Error: ParseError<&'src str>>(
        parser: impl Parser<&'src str, &'src str, Error>,
    ) -> impl Parser<&'src str, Cow<'src, str>, Error> {
        fold_many0(
            parser,
            Cow::default,
            |mut acc: Cow<'src, str>, item: &str| {
                if acc.as_ref().is_empty() {
                    Cow::from(item)
                } else {
                    acc.to_mut().push_str(item);
                    acc
                }
            },
        )
    }
}

impl TemplateParser for HtmlParser {
    fn parse<'src>(&mut self, source: &'src str) -> Result<Vec<Item<'src>>, Error> {
        let (source, items) =
            many0(Self::parse_item)(source).map_err(|e| Error::new(e.to_string()))?;
        eof::<_, nom::error::Error<&'src str>>(source).map_err(|e| Error::new(e.to_string()))?;

        println!("Result: {:?}", items);

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::parser::{Item, TemplateParser};

    use super::HtmlParser;

    #[test]
    fn literal_plain() {
        let mut parser = HtmlParser;

        let result = parser.parse("Hello, World!");

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(result.unwrap(), vec![Item::Literal("Hello, World!")]);
    }

    #[test]
    fn expression() {
        let mut parser = HtmlParser;

        let result = parser.parse("{self.name.to_ascii_uppercase()}");

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(
            result.unwrap(),
            vec![Item::Expression(Cow::from(
                "self.name.to_ascii_uppercase()"
            ))]
        );
    }
}
