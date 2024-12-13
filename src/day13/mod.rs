use aoc_utils::{
    nom::{
        self,
        bytes::complete::tag,
        character::complete::line_ending,
        combinator::map,
        sequence::{pair, preceded, tuple},
    },
    str::StrExt,
    AocError,
};
use num::Rational64;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    let machines = parse(input)?;

    let mut total_tokens = 0;
    for machine in machines.iter() {
        let tokens = find_winning_combination(machine)
            .map(|counts| counts.0.abs() * 3 + counts.1.abs())
            .unwrap_or(0);
        total_tokens += tokens;
    }

    Ok(total_tokens)
}

fn find_winning_combination(machine: &Machine) -> Option<(i64, i64)> {
    let matrix = Matrix::new(machine.a, machine.b);
    let Some(inverse) = matrix.inverse() else {
        tracing::error!(?matrix, "Non-invertible matrix, result may be incorrect");
        return None;
    };

    let detransformed = inverse.mul(&machine.prize);
    if detransformed.x.is_integer() && detransformed.y.is_integer() {
        let button_presses = (detransformed.x.to_integer(), detransformed.y.to_integer());
        Some(button_presses)
    } else {
        None
    }
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    let mut machines = parse(input)?;

    let mut total_tokens = 0;
    for machine in machines.iter_mut() {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
        let tokens = find_winning_combination(machine)
            .map(|counts| counts.0.abs() * 3 + counts.1.abs())
            .unwrap_or(0);
        total_tokens += tokens;
    }

    Ok(total_tokens)
}

fn parse(input: &str) -> anyhow::Result<Vec<Machine>> {
    fn parse_machine(input: &str) -> anyhow::Result<Machine> {
        let a = pair(
            preceded(tag("Button A: X+"), nom::character::complete::i64),
            preceded(tag(", Y+"), nom::character::complete::i64),
        );

        let b = pair(
            preceded(tag("Button B: X+"), nom::character::complete::i64),
            preceded(tag(", Y+"), nom::character::complete::i64),
        );

        let prize = pair(
            preceded(tag("Prize: X="), nom::character::complete::i64),
            preceded(tag(", Y="), nom::character::complete::i64),
        );

        let machine = map(
            tuple((a, line_ending, b, line_ending, prize)),
            |(a, _, b, _, prize)| Machine {
                a: Vector::new(a.0, a.1),
                b: Vector::new(b.0, b.1),
                prize: Vector::new(prize.0, prize.1),
            },
        );

        let result = aoc_utils::parser::parse(machine, input).map_err(|e| {
            tracing::error!(?e);
            AocError::InvalidInput
        })?;

        Ok(result)
    }

    let mut machines = Vec::new();
    for paragraph in input.paragraphs() {
        let machine = parse_machine(paragraph)?;
        machines.push(machine);
    }

    Ok(machines)
}

struct Machine {
    a: Vector,
    b: Vector,
    prize: Vector,
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: Rational64,
    y: Rational64,
}

impl Vector {
    pub fn new(x: impl Into<Rational64>, y: impl Into<Rational64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    m11: Rational64,
    m12: Rational64,
    m21: Rational64,
    m22: Rational64,
}

impl Matrix {
    pub fn new(c1: Vector, c2: Vector) -> Self {
        Self {
            m11: c1.x,
            m12: c1.y,
            m21: c2.x,
            m22: c2.y,
        }
    }

    pub fn mul(&self, vector: &Vector) -> Vector {
        Vector::new(
            vector.x * self.m11 + vector.y * self.m21,
            vector.x * self.m12 + vector.y * self.m22,
        )
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == Rational64::ZERO {
            return None;
        }

        let mul = det.recip();
        let mut adj = self.adjugate();
        adj.m11 *= mul;
        adj.m12 *= mul;
        adj.m21 *= mul;
        adj.m22 *= mul;

        Some(adj)
    }

    pub fn determinant(&self) -> Rational64 {
        self.m11 * self.m22 - self.m21 * self.m12
    }

    pub fn adjugate(&self) -> Self {
        Self {
            m11: self.m22,
            m12: -self.m12,
            m21: -self.m21,
            m22: self.m11,
        }
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
        assert_eq!(result, 480);
    }

    #[test]
    fn test_part_2() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let _result = part_2(EXAMPLE_1).unwrap();
    }
}
