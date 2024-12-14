use aoc_utils::{
    grid::{bitmap::BrailleBitmap, Coordinates, Grid},
    nalgebra,
    nom::{
        self,
        bytes::complete::tag,
        combinator::map,
        sequence::{preceded, separated_pair},
    },
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
    const LIMIT: i64 = 10000;

    let mut robots = parse(input)?;

    let mut max_density = (0, 0);
    for second in 1..=LIMIT {
        step(&mut robots, width, height);
        let density = density_peak(&robots, width, height);

        if density > max_density.0 {
            tracing::trace!(second, density, "New peak density found");
            max_density = (density, second);
        }
    }

    Ok(max_density.1)
}

fn density_peak(robots: &[Robot], width: u64, height: u64) -> i64 {
    const BUCKET_COUNT: usize = 10;

    let mut density = 0;
    let size = Vec2::new(width as i64, height as i64);
    let mut buckets = [0; BUCKET_COUNT * BUCKET_COUNT];
    for robot in robots {
        let bucket_position = (robot.position * BUCKET_COUNT as i64).component_div(&size);
        let bucket_index = (bucket_position.y as usize * BUCKET_COUNT) + bucket_position.x as usize;

        buckets[bucket_index] += 1;
        density = std::cmp::max(density, buckets[bucket_index]);
    }

    density
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
