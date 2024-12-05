use aoc_utils::{hashbrown::HashSet, str::StrExt, AocError};
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    let (rules, updates) = parse(input)?;

    let mut total = 0;
    for update in updates {
        if is_valid_update(&rules, &update.pages) {
            total += update.middle();
        }
    }

    Ok(total)
}

fn is_valid_update(rules: &Rules, update: &[i64]) -> bool {
    for (&first, &second) in update.iter().tuple_combinations() {
        if !rules.is_valid_order(first, second) {
            return false;
        }
    }

    true
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    let (rules, updates) = parse(input)?;

    let mut total = 0;
    for mut update in updates {
        if is_valid_update(&rules, &update.pages) {
            continue;
        }

        fix_update(&rules, &mut update.pages);

        total += update.middle();
    }

    Ok(total)
}

fn fix_update(rules: &Rules, update: &mut [i64]) {
    update.sort_by(|&first, &second| {
        if rules.is_valid_order(first, second) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
}

fn parse(input: &str) -> anyhow::Result<(Rules, Vec<Update>)> {
    let mut paragraphs = input.paragraphs();
    let rules_input = paragraphs.next().ok_or(AocError::InvalidInput)?;
    let updates_input = paragraphs.next().ok_or(AocError::InvalidInput)?;

    let mut rules = Rules::new();
    for line in rules_input.lines() {
        let (first, second) = line.split_once('|').ok_or(AocError::InvalidInput)?;
        let first = i64::from_str_radix(first, 10)?;
        let second = i64::from_str_radix(second, 10)?;
        rules.insert(first, second);
    }

    let mut updates = Vec::new();
    for line in updates_input.lines() {
        let pages = line
            .split(',')
            .map(|part| i64::from_str_radix(part, 10))
            .collect::<Result<Vec<_>, _>>()?;
        updates.push(Update { pages });
    }

    Ok((rules, updates))
}

struct Rules {
    rules: HashSet<(i64, i64)>,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            rules: HashSet::new(),
        }
    }

    pub fn is_valid_order(&self, first: i64, second: i64) -> bool {
        !self.rules.contains(&(second, first))
    }

    pub fn insert(&mut self, first: i64, second: i64) {
        self.rules.insert((first, second));
    }
}

struct Update {
    pages: Vec<i64>,
}

impl Update {
    pub fn middle(&self) -> i64 {
        let index = self.pages.len() / 2;
        self.pages[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_2() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_2(EXAMPLE_1).unwrap();
        assert_eq!(result, 123);
    }
}
