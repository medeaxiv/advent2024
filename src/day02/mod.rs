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
    let mut safe_report_count = 0;
    for report in input.lines() {
        let report = report
            .split(' ')
            .map(|part| i64::from_str_radix(part, 10))
            .collect::<Result<Vec<_>, _>>()?;

        if check_report_is_safe(&report) {
            safe_report_count += 1;
        }
    }

    Ok(safe_report_count)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    let mut safe_report_count = 0;
    'report: for report in input.lines() {
        let report = report
            .split(' ')
            .map(|part| i64::from_str_radix(part, 10))
            .collect::<Result<Vec<_>, _>>()?;

        if check_report_is_safe(&report) {
            safe_report_count += 1;
            continue;
        }

        for filtered_idx in 0..report.len() {
            let mut filtered_report = report.clone();
            filtered_report.remove(filtered_idx);

            if check_report_is_safe(&filtered_report) {
                safe_report_count += 1;
                continue 'report;
            }
        }
    }

    Ok(safe_report_count)
}

fn check_report_is_safe(report: &[i64]) -> bool {
    let mut sign = None;
    for (&a, &b) in report.iter().tuple_windows() {
        let delta = a - b;
        let abs_delta = delta.abs();
        if !(1..=3).contains(&abs_delta) {
            return false;
        }

        let delta_sign = delta.signum();
        if let Some(sign) = sign {
            if sign != delta_sign {
                return false;
            }
        } else {
            sign = Some(delta_sign);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE_1).unwrap();
        assert_eq!(result, 4);
    }
}
