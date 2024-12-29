use allocator_api2::vec::Vec;
use aoc_utils::{
    grid::Coordinates,
    hashbrown::{HashMap, HashSet},
    neighbors::CardinalNeighbors,
    numerics::manhattan_distance,
    AocError,
};

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT, 100));
    builder.add_part(|| part_2(INPUT, 100));
}

fn part_1(input: &str, threshold: i64) -> anyhow::Result<u64> {
    let map = parse(input)?;
    let result = count_shortcuts(&map, threshold, 2);
    Ok(result)
}

fn count_shortcuts(map: &Map, threshold: i64, limit: u32) -> u64 {
    let mut count = 0;
    for (&shortcut_start, start_distance) in map.tiles.iter() {
        for (&node, &end_distance) in map.tiles.iter() {
            let shortcut_len = manhattan_distance(shortcut_start, node);
            if shortcut_len > limit {
                continue;
            }

            let shortcut_distance = end_distance - start_distance - shortcut_len as i64;
            if shortcut_distance >= threshold {
                // tracing::debug!(?shortcut_start, ?node, shortcut_distance);
                count += 1;
            }
        }
    }

    count
}

fn part_2(input: &str, threshold: i64) -> anyhow::Result<u64> {
    let map = parse(input)?;
    let result = count_shortcuts(&map, threshold, 20);
    Ok(result)
}

fn parse(input: &str) -> anyhow::Result<Map> {
    let mut tiles = HashSet::new();
    let mut start = Coordinates::zeros();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coordinates = Coordinates::new(x as u32, y as u32);

            match c {
                '.' | 'E' => {
                    tiles.insert(coordinates);
                }
                'S' => {
                    tiles.insert(coordinates);
                    start = coordinates;
                }
                '#' => {}
                _ => return Err(AocError::InvalidInput.into()),
            }
        }
    }

    let map = Map::new(tiles, start);

    Ok(map)
}

#[derive(Debug, Clone)]
struct Map {
    tiles: HashMap<Coordinates, i64>,
}

impl Map {
    pub fn new(tiles: HashSet<Coordinates>, start: Coordinates) -> Self {
        let mut new = Self {
            tiles: HashMap::with_capacity(tiles.len()),
        };

        let mut stack = Vec::new();
        stack.push((start, 0));

        while let Some((node, distance)) = stack.pop() {
            new.tiles.insert(node, distance);

            for neighbor in CardinalNeighbors::new(node) {
                if tiles.contains(&neighbor) && !new.tiles.contains_key(&neighbor) {
                    stack.push((neighbor, distance + 1))
                }
            }
        }

        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_1(EXAMPLE_1, 10).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part_2() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_2(EXAMPLE_1, 50).unwrap();
        assert_eq!(result, 285);
    }
}
