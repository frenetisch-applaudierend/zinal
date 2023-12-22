use unicode_xid::UnicodeXID;

use crate::parser::input::{Input, Offset};

pub fn parse_rust_identifier<'src>(input: &mut Input<'src>) -> Option<Offset<'src>> {
    let position = input.position();

    let start = input.consume_count(1)?;
    let start_char = start
        .chars()
        .next()
        .expect("Should have exactly 1 character");

    if start_char != '_' && !UnicodeXID::is_xid_start(start_char) {
        input.reset_to(position);
        return None;
    }

    let rest = input.consume_while(UnicodeXID::is_xid_continue);

    if start_char == '_' && rest.is_empty() {
        input.reset_to(position);
        return None;
    }

    Some(input.combine(&[start, rest]))
}

pub fn parse_rust_typename<'src>(input: &mut Input<'src>) -> Option<Offset<'src>> {
    let position = input.position();

    let mut parts = Vec::new();

    if let Some(separator) = input.consume_lit("::") {
        parts.push(separator);
    }

    let Some(ident) = parse_rust_identifier(input) else {
        input.reset_to(position);
        return None;
    };
    parts.push(ident);

    while !input.is_at_end() {
        let Some(separator) = input.consume_lit("::") else {
            break;
        };

        let Some(ident) = parse_rust_identifier(input) else {
            input.reset_to(position);
            return None;
        };

        parts.push(separator);
        parts.push(ident);
    }

    Some(input.combine(&parts))
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        common::parse_rust_typename,
        input::{Input, Offset},
    };

    use super::parse_rust_identifier;

    #[test]
    fn rust_identifier_valid() {
        let valid_identifiers = ["foo", "_foo", "foo2", "_foo2"];

        for ident in valid_identifiers {
            let mut input = Input::new(ident);

            let result = parse_rust_identifier(&mut input);

            let expected = Some(Offset::new(ident, 0));

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn rust_identifier_invalid() {
        let invalid_identifiers = ["", "_", "2foo"];

        for ident in invalid_identifiers {
            let mut input = Input::new(ident);

            let position_before = input.position();
            let result = parse_rust_identifier(&mut input);

            assert_eq!(None, result);
            assert_eq!(position_before, input.position());
        }
    }

    #[test]
    fn rust_typename_valid() {
        let valid_identifiers = ["foo", "Bar", "foo::Bar", "::foo", "::foo::Bar"];

        for ident in valid_identifiers {
            let mut input = Input::new(ident);

            let result = parse_rust_typename(&mut input);

            let expected = Some(Offset::new(ident, 0));

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn rust_typename_invalid() {
        let invalid_identifiers = ["", "::", ":foo", "foo::", ":::foo", "foo:::Bar"];

        for ident in invalid_identifiers {
            let mut input = Input::new(ident);

            let position_before = input.position();
            let result = parse_rust_typename(&mut input);

            assert_eq!(None, result);
            assert_eq!(position_before, input.position());
        }
    }
}
