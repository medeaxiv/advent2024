use regex::Regex;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    let regex = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();

    let mut total = 0;
    for captures in regex.captures_iter(input) {
        let a = captures
            .get(1)
            .expect("Regex has 2 capturing groups")
            .as_str();
        let a = i64::from_str_radix(a, 10)?;
        let b = captures
            .get(2)
            .expect("Regex has 2 capturing groups")
            .as_str();
        let b = i64::from_str_radix(b, 10)?;

        total += a * b;
    }

    Ok(total)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    let regex = Regex::new("do\\(\\)|don't\\(\\)|mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();

    let mut total = 0;
    let mut is_mul_enabled = true;
    for captures in regex.captures_iter(input) {
        let whole_match = captures.get(0).unwrap().as_str();

        if whole_match.starts_with("don't") {
            is_mul_enabled = false;
        } else if whole_match.starts_with("do") {
            is_mul_enabled = true;
        } else if is_mul_enabled && whole_match.starts_with("mul") {
            let a = captures
                .get(1)
                .expect("Regex has 2 capturing groups")
                .as_str();
            let a = i64::from_str_radix(a, 10)?;
            let b = captures
                .get(2)
                .expect("Regex has 2 capturing groups")
                .as_str();
            let b = i64::from_str_radix(b, 10)?;

            total += a * b;
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");
    const EXAMPLE_2: &str = include_str!("example.2.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE_2).unwrap();
        assert_eq!(result, 48)
    }
}
