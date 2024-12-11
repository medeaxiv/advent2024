use aoc_utils::hashbrown::HashMap;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<u64> {
    // It's going to be the lanternfish again, isn't it?
    let mut rocks = parse(input)?;
    for _ in 0..25 {
        step(&mut rocks);
    }

    let total = rocks.iter().map(|(_, count)| count).sum();
    Ok(total)
}

fn step(rocks: &mut Rocks) {
    let original = rocks.clone();
    rocks.clear();

    for (rock, count) in original.iter() {
        if rock.value == 0 {
            rocks.insert(1, count);
        } else if rock.digits % 2 == 0 {
            let (left, right) = split_value(rock.value);
            rocks.insert(left, count);
            rocks.insert(right, count);
        } else {
            rocks.insert(rock.value * 2024, count);
        }
    }
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    // It's the lanternfish again.
    let mut rocks = parse(input)?;
    for _ in 0..75 {
        step(&mut rocks);
    }

    let total = rocks.iter().map(|(_, count)| count).sum();
    Ok(total)
}

fn parse(input: &str) -> anyhow::Result<Rocks> {
    let mut rocks = Rocks::default();
    for span in input.trim().split_whitespace() {
        let value = u64::from_str_radix(span, 10)?;
        rocks.insert(value, 1);
    }

    Ok(rocks)
}

#[derive(Debug, Default, Clone)]
struct Rocks {
    store: HashMap<u64, RockEntry>,
}

impl Rocks {
    pub fn insert(&mut self, value: u64, count: u64) {
        self.store
            .entry(value)
            .and_modify(|entry| entry.count += count)
            .or_insert_with(|| RockEntry::new(value, count));
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (Rock, u64)> + use<'_> {
        self.store.iter().map(|(&value, entry)| {
            (
                Rock {
                    value,
                    digits: entry.digits,
                },
                entry.count,
            )
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    pub value: u64,
    pub digits: u64,
}

#[derive(Debug, Clone, Copy)]
struct RockEntry {
    digits: u64,
    count: u64,
}

impl RockEntry {
    pub fn new(value: u64, count: u64) -> Self {
        Self {
            digits: count_digits(value),
            count,
        }
    }
}

fn split_value(value: u64) -> (u64, u64) {
    let shift = count_digits(value) / 2;

    let mut left = value;
    for _ in 0..shift {
        left /= 10;
    }

    let mut difference = left;
    for _ in 0..shift {
        difference *= 10;
    }

    let right = value - difference;

    (left, right)
}

fn count_digits(mut value: u64) -> u64 {
    if value == 0 {
        return 1;
    }

    let mut digits = 0;
    while value != 0 {
        digits += 1;
        value /= 10;
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part_2() {
        let _result = part_2(EXAMPLE_1).unwrap();
    }
}
