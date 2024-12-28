use aoc_utils::{
    nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, line_ending, space1},
        combinator::{map, value},
        error::ParseError,
        multi::fold_many1,
        sequence::{separated_pair, terminated, tuple},
        IResult,
    },
    AocError,
};

use super::{Device, GateKind, Wire};

pub fn parse(input: &str) -> anyhow::Result<(u64, u64, Device)> {
    let parser = map(
        separated_pair(inputs, line_ending, device),
        |((x, y), device)| (x, y, device),
    );

    match aoc_utils::parser::parse(parser, input) {
        Ok(result) => Ok(result),
        Err(_) => Err(AocError::InvalidInput.into()),
    }
}

fn inputs<'input, E: ParseError<&'input str>>(
    input: &'input str,
) -> IResult<&'input str, (u64, u64), E> {
    let line = terminated(input_bit, line_ending);

    fold_many1(
        line,
        || (0, 0),
        |(x, y), (wire, value)| match wire {
            Wire::X(bit) => (x | (value << bit), y),
            Wire::Y(bit) => (x, y | (value << bit)),
            _ => (x, y),
        },
    )(input)
}

fn device<'input, E: ParseError<&'input str>>(
    input: &'input str,
) -> IResult<&'input str, Device, E> {
    let line = terminated(node, line_ending);

    fold_many1(
        line,
        Device::new,
        |mut device, (output, gate_kind, inputs)| {
            device.insert(output, gate_kind, inputs).ok();
            device
        },
    )(input)
}

fn wire<'input, E: ParseError<&'input str>>(input: &'input str) -> IResult<&'input str, Wire, E> {
    map(alphanumeric1, Wire::new)(input)
}

fn gate_kind<'input, E: ParseError<&'input str>>(
    input: &'input str,
) -> IResult<&'input str, GateKind, E> {
    alt((
        value(GateKind::And, tag("AND")),
        value(GateKind::Or, tag("OR")),
        value(GateKind::Xor, tag("XOR")),
    ))(input)
}

fn bit<'input, E: ParseError<&'input str>>(input: &'input str) -> IResult<&'input str, u64, E> {
    alt((value(0, tag("0")), value(1, tag("1"))))(input)
}

fn input_bit<'input, E: ParseError<&'input str>>(
    input: &'input str,
) -> IResult<&'input str, (Wire, u64), E> {
    separated_pair(wire, tag(": "), bit)(input)
}

fn node<'input, E: ParseError<&'input str>>(
    input: &'input str,
) -> IResult<&'input str, (Wire, GateKind, [Wire; 2]), E> {
    let inputs = map(
        tuple((wire, space1, gate_kind, space1, wire)),
        |(a, _, gate, _, b)| (gate, [a, b]),
    );

    map(
        separated_pair(inputs, tag(" -> "), wire),
        |((gate, inputs), output)| (output, gate, inputs),
    )(input)
}
