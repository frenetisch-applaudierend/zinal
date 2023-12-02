use std::borrow::Cow;

use crate::parser::{
    combinators::{collect_until, insert, literal},
    Keyword,
};

use super::{
    combinators::{take_until, whitespace, Combinator, ParseResult},
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

fn parse_expr<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    literal("{")
        .ignore_then(take_until("}", "}}").map(Item::Expression))
        .then_ignore(literal("}"))
        .parse(input)
}

fn parse_statement<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    select! {
        parse_keyword_statement,
        parse_plain_statement
    }
    .parse(input)
}

fn parse_keyword_statement<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    let Some(start) = keyword_statement_tag(keyword()).parse(input)? else {
        return Ok(None);
    };

    let body = if start.keyword.requires_body() {
        body()
            .then_ignore(keyword_statement_tag(literal("end")).optional())
            .parse(input)?
            .ok_or(Error::new("Expected body after block statement"))?
    } else {
        vec![]
    };

    return Ok(Some(Item::KeywordStatement {
        keyword: start.keyword,
        statement: start.statement,
        body,
    }));

    #[derive(Debug, Clone)]
    struct KeywordStatementTag<'src, T> {
        keyword: T,
        statement: Option<Cow<'src, str>>,
    }

    fn keyword_statement_tag<'src, T: Clone>(
        keyword: impl Combinator<'src, Output = T> + Clone,
    ) -> impl Combinator<'src, Output = KeywordStatementTag<'src, T>> + Clone
    where
        T: Clone,
    {
        let statement = take_until("#>", "##>").map(Some);

        let longform = literal("<#")
            .ignore_then(whitespace().optional())
            .ignore_then(keyword.clone())
            .then_ignore(whitespace())
            .then(statement)
            .then_ignore(literal("#>"));

        let shortform = literal("<#")
            .ignore_then(whitespace().optional())
            .ignore_then(keyword)
            .then(insert(None))
            .then_ignore(whitespace().optional())
            .then_ignore(select!(literal(">"), literal("#>")));

        select! {
            longform,
            shortform
        }
        .map(|(keyword, statement)| KeywordStatementTag { keyword, statement })
    }

    fn block_statement_end<'src>() -> impl Combinator<'src, Output = ()> + Clone {
        let end_keyword = select!(
            literal("end").map(|_| ()),
            keyword()
                .filter(|k| *k == Keyword::Else || *k == Keyword::ElseIf)
                .map(|_| ())
        );

        keyword_statement_tag(end_keyword).map(|_| ())
    }

    fn keyword<'src>() -> impl Combinator<'src, Output = Keyword> + Clone {
        select! {
            literal("if").map(|_| Keyword::If),
            literal("else").then(whitespace()).then(literal("if")).map(|_| Keyword::ElseIf),
            literal("else").map(|_| Keyword::Else),
            literal("for").map(|_| Keyword::For),
            literal("while").map(|_| Keyword::While),
            literal("loop").map(|_| Keyword::Loop),
            literal("break").map(|_| Keyword::Break),
            literal("continue").map(|_| Keyword::Continue),
            literal("let").map(|_| Keyword::Let)
        }
    }

    fn body<'src>() -> impl Combinator<'src, Output = Vec<Item<'src>>> {
        |input: &mut Input<'src>| {
            let Some(body) = collect_until(parse_item, block_statement_end()).parse(input)? else {
                return Err(Error::unexpected_eof());
            };

            Ok(Some(body))
        }
    }
}

fn parse_plain_statement<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    literal("<#")
        .ignore_then(whitespace().optional())
        .ignore_then(take_until("#>", "##>"))
        .then_ignore(literal("#>"))
        .map(Item::PlainStatement)
        .parse(input)
}

fn parse_component<'src>(_input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    Ok(None)
}

fn parse_literal<'src>(input: &mut Input<'src>) -> ParseResult<Item<'src>> {
    // consume possible leading < or {
    let Some(lead) = input.consume_count(1) else {
        return Err(Error::unexpected_eof());
    };
    let rest = input
        .consume_until_any("<{")
        .unwrap_or_else(|| input.consume_all());
    let combined = input.combine(&[lead, rest]);

    Ok(Some(Item::Literal(combined.into_cow())))
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::parser::{input::Input, Item, Keyword, TemplateParser};

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

    #[test]
    fn literal_with_expression() {
        let mut parser = HtmlParser;

        let input = Input::new("<div>{self.name.to_ascii_uppercase()}</div>");
        let result = parser.parse(input);

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(
            result.unwrap(),
            vec![
                Item::Literal(Cow::from("<div>")),
                Item::Expression(Cow::from("self.name.to_ascii_uppercase()")),
                Item::Literal(Cow::from("</div>"))
            ]
        );
    }

    #[test]
    fn expression_then_literal() {
        let mut parser = HtmlParser;

        let input = Input::new("{self.name} is here");
        let result = parser.parse(input);

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(
            result.unwrap(),
            vec![
                Item::Expression(Cow::from("self.name")),
                Item::Literal(Cow::from(" is here"))
            ]
        );
    }

    #[test]
    fn plain_statement() {
        let mut parser = HtmlParser;

        let input = Input::new("<# println!(\"Hello, {}\", self.name) #>");
        let result = parser.parse(input);

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(
            result.unwrap(),
            vec![Item::PlainStatement(Cow::from(
                "println!(\"Hello, {}\", self.name) "
            ))]
        );
    }

    #[test]
    fn block_statement_for() {
        let mut parser = HtmlParser;

        let input = Input::new("<# for name in self.names #>Hello, {name}<#end>");
        let result = parser.parse(input);

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(
            result.unwrap(),
            vec![Item::KeywordStatement {
                keyword: Keyword::For,
                statement: Some(Cow::from("name in self.names ")),
                body: vec![
                    Item::Literal(Cow::from("Hello, ")),
                    Item::Expression(Cow::from("name"))
                ]
            }]
        );
    }

    #[test]
    fn block_statement_if() {
        let mut parser = HtmlParser;

        let input = Input::new(
            "<#if age > 18 #>Over 18<# else if age < 18 #>Under 18<#else>Exactly 18<#end>",
        );
        let result = parser.parse(input);

        assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
        assert_eq!(
            result.unwrap(),
            vec![
                Item::KeywordStatement {
                    keyword: Keyword::If,
                    statement: Some(Cow::from("age > 18 ")),
                    body: vec![Item::Literal(Cow::from("Over 18")),]
                },
                Item::KeywordStatement {
                    keyword: Keyword::ElseIf,
                    statement: Some(Cow::from("age < 18 ")),
                    body: vec![Item::Literal(Cow::from("Under 18")),]
                },
                Item::KeywordStatement {
                    keyword: Keyword::Else,
                    statement: None,
                    body: vec![Item::Literal(Cow::from("Exactly 18")),]
                }
            ]
        );
    }
}
