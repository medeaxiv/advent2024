use aoc_utils::{
    grid::{bitmap::BrailleBitmap, Grid},
    hashbrown::HashSet,
    nalgebra,
    neighbors::CardinalNeighbors,
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

    let mut detect_tree_context = DetectTreeContext::default();
    for second in 1..=LIMIT {
        step(&mut robots, width, height);
        if detect_tree(&robots, &mut detect_tree_context) {
            return Ok(second);
        }
    }

    let message = format!("Unable to find tree in {LIMIT} seconds");
    Err(AocError::message(message).into())
}

#[derive(Default)]
#[allow(unused)]
struct DetectTreeContext {
    map: HashSet<Vec2>,
    stack: Vec<(Vec2, i64)>,
    visited: HashSet<Vec2>,
}

fn detect_tree(robots: &[Robot], ctx: &mut DetectTreeContext) -> bool {
    // Adjust the threshold for your input
    const CLUSTER_SIZE_THRESHOLD: i64 = 15;

    // Somehow reusing the visited set allocation is noticeably faster on my machine,
    // but not reusing the map and stack.

    let mut map = HashSet::new();
    // let map = &mut ctx.map;
    // map.clear();
    map.extend(robots.iter().map(|robot| robot.position));

    let mut stack = Vec::new();
    // let stack = &mut ctx.stack;
    // stack.clear();

    // let mut visited = HashSet::new();
    let visited = &mut ctx.visited;
    visited.clear();

    for &position in map.iter() {
        stack.push((position, 1));

        while let Some((node, cluster_size)) = stack.pop() {
            if cluster_size >= CLUSTER_SIZE_THRESHOLD {
                // Use this while adjusting the threshold for your input
                // print_map(&map);
                return true;
            }

            if !visited.insert(node) {
                continue;
            }

            let neighbors = CardinalNeighbors::new(node)
                .filter(|neighbor| map.contains(neighbor))
                .map(|neighbor| (neighbor, cluster_size + 1));
            stack.extend(neighbors)
        }
    }

    false
}

#[allow(dead_code)]
fn print_map(map: &HashSet<Vec2>) {
    let mut grid = Grid::<bool>::new(101, 103);
    for i in 0..grid.len() {
        let coordinates = grid.get_coordinates(i).unwrap();
        let coordinates = Vec2::new(coordinates.x as i64, coordinates.y as i64);
        grid[i] = map.contains(&coordinates);
    }

    let bitmap = BrailleBitmap(grid);
    println!("{bitmap}\n------------------------------------");
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
