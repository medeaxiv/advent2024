use aoc_utils::{hashbrown::HashMap, AocError};
use itertools::Itertools;
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
    let numbers = parse(input)?;

    let result = numbers
        .par_iter()
        .map(|&n| {
            SecretNumberIter(n)
                .nth(2000)
                .expect("SecretNumberIter is an infinite iterator")
        })
        .sum();

    Ok(result)
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let numbers = parse(input)?;

    let result = numbers
        .par_iter()
        .map(|&num| all_sequences(num, 2000))
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (&key, &value) in b.iter() {
                    a.entry(key).and_modify(|a| *a += value).or_insert(value);
                }

                a
            },
        );

    let (_, &bananas) = result
        .iter()
        .max_by_key(|(_, &total)| total)
        .ok_or(AocError::EmptyInput)?;
    Ok(bananas)
}

fn all_sequences(secret_number: u64, len: usize) -> HashMap<PriceDeltaSequence, u64> {
    let prices = SecretNumberIter(secret_number)
        .map(price)
        .take(len)
        .collect::<Vec<_>>();

    let mut map = HashMap::new();
    for price_sequence in prices.iter().copied().tuple_windows() {
        let key = PriceDeltaSequence::from_price_sequence(price_sequence);
        let price = price_sequence.4;
        map.entry(key).or_insert(price as u64);
    }

    map
}

fn parse(input: &str) -> anyhow::Result<Vec<u64>> {
    let numbers = input
        .lines()
        .map(|line| u64::from_str_radix(line, 10))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(numbers)
}

fn price(number: u64) -> i8 {
    (number % 10) as i8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PriceDeltaSequence([i8; 4]);

impl PriceDeltaSequence {
    #[inline]
    pub fn new(a: i8, b: i8, c: i8, d: i8) -> Self {
        Self([a, b, c, d])
    }

    pub fn from_price_sequence((a, b, c, d, e): (i8, i8, i8, i8, i8)) -> Self {
        Self::new(b - a, c - b, d - c, e - d)
    }
}

fn next_secret_number(mut number: u64) -> u64 {
    number = ((number * 64) ^ number) & 0x00ffffff;
    number = ((number / 32) ^ number) & 0x00ffffff;
    number = ((number * 2048) ^ number) & 0x00ffffff;
    number
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SecretNumberIter(pub u64);

impl Iterator for SecretNumberIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let result = self.0;
        self.0 = next_secret_number(self.0);
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");
    const EXAMPLE_2: &str = include_str!("example.2.txt");

    #[test]
    fn test_next_secret_number() {
        let result = SecretNumberIter(123).take(11).collect::<Vec<_>>();
        assert_eq!(
            result,
            [
                123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
                7753432, 5908254,
            ]
        )
    }

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 37327623);
    }

    #[test]
    fn test_part_2() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_2(EXAMPLE_2).unwrap();
        assert_eq!(result, 23);
    }
}
