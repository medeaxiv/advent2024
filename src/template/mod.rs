use aoc_utils::AocError;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(_input: &str) -> anyhow::Result<i64> {
    Err(AocError::Todo.into())
}

fn part_2(_input: &str) -> anyhow::Result<i64> {
    Err(AocError::Todo.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let _result = part_1(EXAMPLE_1).unwrap();
    }

    #[test]
    fn test_part_2() {
        let _result = part_2(EXAMPLE_1).unwrap();
    }
}
