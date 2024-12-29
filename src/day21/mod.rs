use aoc_utils::{cache::Cache, hashbrown::HashMap};

use self::{
    keypad::{Dpad, DpadButton, Numpad, NumpadButton},
    path::Path,
};

mod keypad;
mod path;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<u64> {
    let codes = parse(input)?;

    let mut total = 0;
    let mut cache = HashMap::new();
    for code in codes.iter() {
        let length = code_length(code, 3, &mut cache);
        total += code.value * length;
    }

    Ok(total)
}

fn code_length(code: &Code, dpads: usize, cache: &mut impl Cache<(Path, usize), u64>) -> u64 {
    let mut length = 0;
    let mut current = NumpadButton::Accept;
    for &next in code.buttons.iter() {
        let path = Numpad::path_between(current, next);
        length += path_length(path, dpads, cache);
        current = next;
    }

    length
}

fn path_length(path: Path, dpads: usize, cache: &mut impl Cache<(Path, usize), u64>) -> u64 {
    match dpads {
        0 => return 0,
        1 => return path.len() as u64 + 1,
        _ => {}
    }

    if let Some(&length) = cache.get(&(path, dpads)) {
        return length;
    }

    let mut length = 0;
    let mut current = DpadButton::Accept;
    for &direction in path.iter() {
        let next = DpadButton::Direction(direction);
        let subpath = Dpad::path_between(current, next);
        length += path_length(subpath, dpads - 1, cache);
        current = next;
    }

    length += {
        let subpath = Dpad::path_between(current, DpadButton::Accept);
        path_length(subpath, dpads - 1, cache)
    };

    cache.insert((path, dpads), length);
    length
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let codes = parse(input)?;

    let mut total = 0;
    let mut cache = HashMap::new();
    for code in codes.iter() {
        let length = code_length(code, 26, &mut cache);
        total += code.value * length;
    }

    Ok(total)
}

fn parse(input: &str) -> anyhow::Result<Vec<Code>> {
    fn parse_code(input: &str) -> anyhow::Result<Code> {
        let mut code = Code::default();

        for c in input.chars() {
            if c.is_ascii_digit() {
                let value = c.to_digit(10).unwrap() as u64;
                code.value = code.value * 10 + value;
            }

            let button = NumpadButton::from_char(c)?;
            code.buttons.push(button);
        }

        Ok(code)
    }

    let mut codes = Vec::new();
    for line in input.lines() {
        let code = parse_code(line)?;
        codes.push(code);
    }

    Ok(codes)
}

#[derive(Debug, Default, Clone)]
struct Code {
    buttons: Vec<NumpadButton>,
    value: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let _result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(_result, 126384)
    }

    #[test]
    fn test_part_2() {
        let _result = part_2(EXAMPLE_1).unwrap();
    }
}
// v<<A>>^A<A>AvA<^AA>A<vAAA>^A
