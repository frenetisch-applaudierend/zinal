use crate::parser::input::Input;

use super::ParseResult;

pub fn select2<'src>(
    input: &mut Input<'src>,
    parsers: (
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
    ),
) -> ParseResult<'src> {
    let (p1, p2) = parsers;
    if let Some(result) = p1(input)? {
        return Ok(Some(result));
    }

    p2(input)
}

pub fn select3<'src>(
    input: &mut Input<'src>,
    parsers: (
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
    ),
) -> ParseResult<'src> {
    let (p1, p2, p3) = parsers;
    if let Some(result) = p1(input)? {
        return Ok(Some(result));
    }

    return select2(input, (p2, p3));
}

pub fn select4<'src>(
    input: &mut Input<'src>,
    parsers: (
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
    ),
) -> ParseResult<'src> {
    let (p1, p2, p3, p4) = parsers;
    if let Some(result) = p1(input)? {
        return Ok(Some(result));
    }

    return select3(input, (p2, p3, p4));
}

pub fn select5<'src>(
    input: &mut Input<'src>,
    parsers: (
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
    ),
) -> ParseResult<'src> {
    let (p1, p2, p3, p4, p5) = parsers;
    if let Some(result) = p1(input)? {
        return Ok(Some(result));
    }

    return select4(input, (p2, p3, p4, p5));
}

pub fn select6<'src>(
    input: &mut Input<'src>,
    parsers: (
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
        impl FnOnce(&mut Input<'src>) -> ParseResult<'src>,
    ),
) -> ParseResult<'src> {
    let (p1, p2, p3, p4, p5, p6) = parsers;
    if let Some(result) = p1(input)? {
        return Ok(Some(result));
    }

    return select5(input, (p2, p3, p4, p5, p6));
}
