use aoc_utils::{hashbrown::HashMap, AocError};

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in input.lines() {
        let (a_part, b_part) = line.split_once("   ").ok_or(AocError::InvalidInput)?;
        let a_value = i64::from_str_radix(a_part, 10)?;
        let b_value = i64::from_str_radix(b_part, 10)?;

        a.push(a_value);
        b.push(b_value);
    }

    a.sort();
    b.sort();

    let total_distance = Iterator::zip(a.iter(), b.iter())
        .map(|(&a, &b)| (a - b).abs())
        .sum();

    Ok(total_distance)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    let mut list = Vec::new();
    let mut counts = HashMap::new();

    for line in input.lines() {
        let (a_part, b_part) = line.split_once("   ").ok_or(AocError::InvalidInput)?;
        let a_value = i64::from_str_radix(a_part, 10)?;
        let b_value = i64::from_str_radix(b_part, 10)?;

        list.push(a_value);
        counts
            .entry(b_value)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    }

    let similarity_score = list
        .iter()
        .map(|num| *num * counts.get(num).copied().unwrap_or(0))
        .sum();

    Ok(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE_1).unwrap();
        assert_eq!(result, 31);
    }
}
