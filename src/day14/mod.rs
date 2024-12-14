use aoc_utils::{
    grid::{bitmap::BrailleBitmap, Coordinates, Grid},
    nalgebra,
    nom::{
        self,
        bytes::complete::tag,
        combinator::map,
        sequence::{preceded, separated_pair},
    },
    numerics::Congruence,
    AocError,
};

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT, 101, 103));
    builder.add_part(|| part_2(INPUT, 101, 103));
}

fn part_1(input: &str, width: u64, height: u64) -> anyhow::Result<i64> {
    let mut robots = parse(input)?;
    for _ in 0..100 {
        step(&mut robots, width, height);
    }

    let result = security_factor(&robots, width, height);
    Ok(result)
}

fn step(robots: &mut [Robot], width: u64, height: u64) {
    for robot in robots {
        robot.position += robot.velocity;
        robot.position.x = robot.position.x.rem_euclid(width as i64);
        robot.position.y = robot.position.y.rem_euclid(height as i64);
    }
}

fn security_factor(robots: &[Robot], width: u64, height: u64) -> i64 {
    let offset = Vec2::new(width as i64, height as i64) / 2;

    let mut quadrants = [0; 4];
    for robot in robots {
        let offset_position = robot.position - offset;
        let quadrant_selector = (offset_position.x.signum(), offset_position.y.signum());
        let quadrant = match quadrant_selector {
            (0, _) | (_, 0) => continue,
            (-1, -1) => 0,
            (1, -1) => 1,
            (-1, 1) => 2,
            (1, 1) => 3,
            _ => unreachable!(),
        };

        quadrants[quadrant] += 1;
    }

    quadrants.into_iter().product()
}

fn part_2(input: &str, width: u64, height: u64) -> anyhow::Result<i64> {
    let mut robots = parse(input)?;

    let mut min_x_variance = (i64::MAX, 0);
    let mut min_y_variance = (i64::MAX, 0);
    let limit = std::cmp::max(width, height) as i64;
    for second in 1..=limit {
        step(&mut robots, width, height);

        let variance = position_variance(&robots);
        if variance.x < min_x_variance.0 {
            min_x_variance = (variance.x, second);
        }

        if variance.y < min_y_variance.0 {
            min_y_variance = (variance.y, second);
        }
    }

    let congruences = [
        Congruence {
            remainder: min_x_variance.1,
            modulus: width as i64,
        },
        Congruence {
            remainder: min_y_variance.1,
            modulus: height as i64,
        },
    ];

    // I blatantly stole this from the advent of code subreddit. Look at the previous `Day 14`
    // commits to see my actual solution.
    if let Some(result) = aoc_utils::numerics::crt(congruences) {
        Ok(result.remainder)
    } else {
        Err(AocError::message("Unable to find a tree").into())
    }
}

fn position_variance(robots: &[Robot]) -> Vec2 {
    let mean = {
        let sum = robots.iter().map(|robot| robot.position).sum::<Vec2>();
        sum / robots.len() as i64
    };

    let sum = robots
        .iter()
        .map(|robot| robot.position - mean)
        .map(|delta| delta.component_mul(&delta))
        .sum::<Vec2>();
    sum / robots.len() as i64
}

#[allow(dead_code)]
fn print_map(robots: &[Robot], width: u64, height: u64) {
    let mut grid = Grid::<bool>::new(width as u32, height as u32);

    for robot in robots {
        let coordinates = Coordinates::new(robot.position.x as u32, robot.position.y as u32);
        grid[coordinates] = true;
    }

    let bitmap = BrailleBitmap(grid);
    println!("{bitmap}");
}

fn parse(input: &str) -> anyhow::Result<Vec<Robot>> {
    fn parse_robot(input: &str) -> anyhow::Result<Robot> {
        let position = preceded(
            tag("p="),
            separated_pair(
                nom::character::complete::i64,
                tag(","),
                nom::character::complete::i64,
            ),
        );

        let velocity = preceded(
            tag("v="),
            separated_pair(
                nom::character::complete::i64,
                tag(","),
                nom::character::complete::i64,
            ),
        );

        let robot = map(separated_pair(position, tag(" "), velocity), |(p, v)| {
            Robot {
                position: Vec2::new(p.0, p.1),
                velocity: Vec2::new(v.0, v.1),
            }
        });

        let result = aoc_utils::parser::parse(robot, input).map_err(|e| {
            tracing::error!(?e);
            AocError::InvalidInput
        })?;

        Ok(result)
    }

    let mut robots = Vec::new();
    for line in input.lines() {
        let robot = parse_robot(line)?;
        robots.push(robot);
    }

    Ok(robots)
}

type Vec2 = nalgebra::Vector2<i64>;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Vec2,
    velocity: Vec2,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1, 11, 7).unwrap();
        assert_eq!(result, 12);
    }
}
