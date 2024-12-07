use aoc_utils::AocError;
use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<u64> {
    let equations = parse(input)?;

    let calibration = equations
        .par_iter()
        .filter(|equation| can_be_represented(equation))
        .map(Equation::value)
        .sum();

    Ok(calibration)
}

fn can_be_represented(equation: &Equation) -> bool {
    fn recurse(equation: &Equation, current: u64, count: usize) -> bool {
        if current > equation.value() {
            return false;
        } else if current == equation.value() && count == equation.len() {
            return true;
        }

        if let Some(next) = equation.operand(count) {
            recurse(equation, current + next, count + 1)
                || recurse(equation, current * next, count + 1)
        } else {
            false
        }
    }

    if let Some(first) = equation.operand(0) {
        recurse(equation, first, 1)
    } else {
        equation.value == 0
    }
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let equations = parse(input)?;

    let calibration = equations
        .par_iter()
        .filter(|equation| can_be_represented_with_concatenation(equation))
        .map(Equation::value)
        .sum();

    Ok(calibration)
}

fn can_be_represented_with_concatenation(equation: &Equation) -> bool {
    fn concatenate(first: u64, second: u64) -> u64 {
        let mut shift = (0, second);
        while shift.1 > 0 {
            shift.0 += 1;
            shift.1 /= 10;
        }

        first * 10u64.pow(shift.0) + second
    }

    fn recurse(equation: &Equation, current: u64, count: usize) -> bool {
        if current > equation.value() {
            return false;
        } else if current == equation.value() && count == equation.len() {
            return true;
        }

        if let Some(next) = equation.operand(count) {
            recurse(equation, current + next, count + 1)
                || recurse(equation, current * next, count + 1)
                || recurse(equation, concatenate(current, next), count + 1)
        } else {
            false
        }
    }

    if let Some(first) = equation.operand(0) {
        recurse(equation, first, 1)
    } else {
        equation.value == 0
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Equation>> {
    let mut equations = Vec::new();
    for line in input.lines() {
        let (value, operands) = line.split_once(": ").ok_or(AocError::InvalidInput)?;
        let value = u64::from_str_radix(value, 10)?;
        let operands = operands
            .split_whitespace()
            .map(|part| u64::from_str_radix(part, 10))
            .collect::<Result<Vec<_>, _>>()?;

        let equation = Equation { value, operands };
        equations.push(equation);
    }

    Ok(equations)
}

#[derive(Debug, Clone)]
struct Equation {
    value: u64,
    operands: Vec<u64>,
}

impl Equation {
    pub fn len(&self) -> usize {
        self.operands.len()
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn operand(&self, index: usize) -> Option<u64> {
        self.operands.get(index).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE_1).unwrap();
        assert_eq!(result, 11387);
    }
}
