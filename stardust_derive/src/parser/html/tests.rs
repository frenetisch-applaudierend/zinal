use std::borrow::Cow;

use parser_common::Input;

use crate::parser::{Item, Keyword, TemplateArgument, TemplateArgumentValue, TemplateParser};

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
fn expression_with_escape_and_whitespace() {
    let mut parser = HtmlParser;

    let input = Input::new("{ format!(\"{}}\", foo) }");
    let result = parser.parse(input);

    assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
    assert_eq!(
        result.unwrap(),
        vec![Item::Expression(Cow::from("format!(\"{}\", foo)"))]
    );
}

#[test]
fn expression_unterminated() {
    let mut parser = HtmlParser;

    let input = Input::new("{ format!(\"{}}\", foo) ");
    let result = parser.parse(input);

    assert!(result.is_err(), "Unexpectedly succeeded");
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
            "println!(\"Hello, {}\", self.name)"
        ))]
    );
}

#[test]
fn plain_statement_with_escape() {
    let mut parser = HtmlParser;

    let input = Input::new("<# println!(\"Hello, <# Nested ##>\") #>");
    let result = parser.parse(input);

    assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
    assert_eq!(
        result.unwrap(),
        vec![Item::PlainStatement(Cow::from(
            "println!(\"Hello, <# Nested #>\")"
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

    let input =
        Input::new("<#if age > 18 #>Over 18<# else if age < 18 #>Under 18<#else>Exactly 18<#end>");
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

#[test]
fn child_template() {
    let mut parser = HtmlParser;

    let input = Input::new(
        "<Child expr={self.name} lit_double=\"test double\" lit_single='test single' />",
    );
    let result = parser.parse(input);

    assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
    assert_eq!(
        result.unwrap(),
        vec![Item::ChildTemplate {
            name: Cow::from("Child"),
            arguments: vec![
                TemplateArgument {
                    name: "expr",
                    value: TemplateArgumentValue::Expression(Cow::from("self.name"))
                },
                TemplateArgument {
                    name: "lit_double",
                    value: TemplateArgumentValue::Literal(Cow::from("test double"))
                },
                TemplateArgument {
                    name: "lit_single",
                    value: TemplateArgumentValue::Expression(Cow::from("test single"))
                }
            ],
            children: vec![]
        }]
    );
}
