use std::borrow::Cow;

use crate::parser::{input::Input, Item, Keyword, TemplateArgument, TemplateArgumentValue};

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
fn keyword_statement_shorthand() {
    let mut parser = HtmlParser;

    let input = Input::new("<#break>");
    let result = parser.parse(input);

    assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
    assert_eq!(
        result.unwrap(),
        vec![Item::KeywordStatement {
            keyword: Keyword::Break,
            statement: None,
            body: Vec::new()
        }]
    );
}

#[test]
fn keyword_statement_longform() {
    let mut parser = HtmlParser;

    let input = Input::new("<# break 'outer #>");
    let result = parser.parse(input);

    assert!(result.is_ok(), "Error in result: {:?}", result.unwrap_err());
    assert_eq!(
        result.unwrap(),
        vec![Item::KeywordStatement {
            keyword: Keyword::Break,
            statement: Some(Cow::Borrowed("'outer")),
            body: Vec::new()
        }]
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
            statement: Some(Cow::from("name in self.names")),
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
                statement: Some(Cow::from("age > 18")),
                body: vec![Item::Literal(Cow::from("Over 18")),]
            },
            Item::KeywordStatement {
                keyword: Keyword::ElseIf,
                statement: Some(Cow::from("age < 18")),
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
fn comment() {
    let mut parser = HtmlParser;

    let input = Input::new("<!-- Comment {expr} <#end> -->");

    let result = parser.parse(input).expect("Should have parsed");

    assert_eq!(
        result,
        vec![Item::Literal(Cow::from("<!-- Comment {expr} <#end> -->"))]
    );
}

#[test]
fn child_template_minimal() {
    let mut parser = HtmlParser;

    let input = Input::new("<::foo::Bar />");

    let result = parser.parse(input).expect("Should have parsed");

    assert_eq!(
        result,
        vec![Item::ChildTemplate {
            name: Cow::from("::foo::Bar"),
            arguments: vec![],
            children: vec![]
        }]
    );
}

#[test]
fn child_template_with_args() {
    let mut parser = HtmlParser;

    let input = Input::new(
        "<Child expr={self.name} lit_double=\"test double\" lit_single='test single' />",
    );
    let result = parser.parse(input).expect("Should have parsed");

    assert_eq!(
        result,
        vec![Item::ChildTemplate {
            name: Cow::from("Child"),
            arguments: vec![
                TemplateArgument {
                    name: Cow::from("expr"),
                    value: TemplateArgumentValue::Expression(Cow::from("self.name"))
                },
                TemplateArgument {
                    name: Cow::from("lit_double"),
                    value: TemplateArgumentValue::Literal(Cow::from("test double"))
                },
                TemplateArgument {
                    name: Cow::from("lit_single"),
                    value: TemplateArgumentValue::Literal(Cow::from("test single"))
                }
            ],
            children: vec![]
        }]
    );
}

#[test]
fn child_template_with_args_and_body() {
    let mut parser = HtmlParser;

    let input = Input::new("<Child expr={self.name}>Hello, World!</Child>");
    let result = parser.parse(input).expect("Should have parsed");

    assert_eq!(
        result,
        vec![Item::ChildTemplate {
            name: Cow::from("Child"),
            arguments: vec![TemplateArgument {
                name: Cow::from("expr"),
                value: TemplateArgumentValue::Expression(Cow::from("self.name"))
            },],
            children: vec![Item::Literal(Cow::from("Hello, World!"))]
        }]
    );
}
