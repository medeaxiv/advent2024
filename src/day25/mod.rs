use aoc_utils::{nalgebra, str::StrExt, AocError};
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let (keys, locks) = parse(input)?;

    let result = keys
        .iter()
        .cartesian_product(locks.iter())
        .filter(|(key, lock)| {
            let sum = *key + *lock;
            let max = sum.max();
            max <= 7
        })
        .count();

    Ok(result)
}

fn parse(input: &str) -> anyhow::Result<(Vec<Vector>, Vec<Vector>)> {
    fn entry_value(input: &str) -> anyhow::Result<Vector> {
        let mut value = Vector::zeros();

        for line in input.lines() {
            for (index, c) in line.chars().enumerate() {
                if index >= value.len() {
                    return Err(AocError::InvalidInput.into());
                }

                if c == '#' {
                    value[index] += 1;
                }
            }
        }

        Ok(value)
    }

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for entry in input.paragraphs() {
        let value = entry_value(entry)?;

        if entry.starts_with("#####") {
            keys.push(value);
        } else {
            locks.push(value);
        }
    }

    Ok((keys, locks))
}

type Vector = nalgebra::Vector5<i8>;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let _result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(_result, 3);
    }
}
