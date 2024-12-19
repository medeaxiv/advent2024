use aoc_utils::{cache::Cache, AocError};
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
    let (towels, patterns) = parse(input)?;

    let result = patterns
        .par_iter()
        .filter(|pattern| can_be_represented(pattern, &towels))
        .count() as u64;

    Ok(result)
}

fn can_be_represented(pattern: &str, towels: &[&str]) -> bool {
    fn recurse(pattern: &str, towels: &[&str], cache: &mut impl Cache<usize, bool>) -> bool {
        if let Some(&cached) = cache.get(&pattern.len()) {
            return cached;
        }

        let mut can_be_represented = false;
        for towel in towels {
            if let Some(remaining) = pattern.strip_prefix(towel) {
                can_be_represented = recurse(remaining, towels, cache);
                if can_be_represented {
                    break;
                }
            }
        }

        cache.insert(pattern.len(), can_be_represented);
        can_be_represented
    }

    let mut cache = vec![None; pattern.len() + 1];
    cache[0] = Some(true);

    recurse(pattern, towels, &mut cache);

    cache[pattern.len()].unwrap_or(false)
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let (towels, patterns) = parse(input)?;

    let result = patterns
        .par_iter()
        .map(|pattern| count_representations(pattern, &towels))
        .sum();

    Ok(result)
}

fn count_representations(pattern: &str, towels: &[&str]) -> u64 {
    fn recurse(pattern: &str, towels: &[&str], cache: &mut impl Cache<usize, u64>) -> u64 {
        if let Some(&cached) = cache.get(&pattern.len()) {
            return cached;
        }

        let mut count = 0;
        for towel in towels {
            if let Some(remaining) = pattern.strip_prefix(towel) {
                count += recurse(remaining, towels, cache);
            }
        }

        cache.insert(pattern.len(), count);
        count
    }

    let mut cache = vec![None; pattern.len() + 1];
    cache[0] = Some(1);

    recurse(pattern, towels, &mut cache);

    cache[pattern.len()].unwrap_or(0)
}

fn parse(input: &str) -> anyhow::Result<(Vec<&str>, Vec<&str>)> {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .ok_or(AocError::InvalidInput)?
        .split(", ")
        .collect();

    lines.next();

    let patterns = lines.collect();

    Ok((towels, patterns))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE_1).unwrap();
        assert_eq!(result, 16);
    }
}
