use std::collections::VecDeque;

use allocator_api2::vec::Vec;
use aoc_utils::{
    grid::{Coordinates, Grid},
    hashbrown::HashSet,
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
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let mut count = 0;
    for &shortcut_start in map.path.iter() {
        let Tile::Path(start_distance) = map.grid[shortcut_start] else {
            continue;
        };

        queue.push_back(shortcut_start);
        while let Some(node) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }

            let shortcut_len = manhattan_distance(shortcut_start, node);
            if shortcut_len > limit {
                break;
            }

            if let Some(&Tile::Path(end_distance)) = map.grid.get(node) {
                let shortcut_distance = end_distance - start_distance - shortcut_len as i64;
                if shortcut_distance >= threshold {
                    // tracing::debug!(?shortcut_start, ?node, shortcut_distance);
                    count += 1;
                }
            }

            for neighbor in CardinalNeighbors::new(node) {
                queue.push_back(neighbor);
            }
        }

        queue.clear();
        visited.clear();
    }

    count
}

fn part_2(input: &str, threshold: i64) -> anyhow::Result<u64> {
    let map = parse(input)?;
    let result = count_shortcuts(&map, threshold, 20);
    Ok(result)
}

fn parse(input: &str) -> anyhow::Result<Map> {
    let line_width = input
        .lines()
        .next()
        .ok_or(AocError::InvalidInput)?
        .chars()
        .count();

    let mut lines = 0;
    let mut data = Vec::with_capacity(line_width * line_width);
    let mut start = Coordinates::zeros();
    let mut end = Coordinates::zeros();
    for (y, line) in input.lines().enumerate() {
        lines += 1;

        for (x, c) in line.chars().enumerate() {
            let coordinates = Coordinates::new(x as u32, y as u32);

            match c {
                '#' => data.push(Tile::Wall),
                '.' => data.push(Tile::Path(-1)),
                'S' => {
                    data.push(Tile::Path(-1));
                    start = coordinates;
                }
                'E' => {
                    data.push(Tile::Path(-1));
                    end = coordinates;
                }
                _ => return Err(AocError::InvalidInput.into()),
            }
        }
    }

    let grid = Grid::from_vec(line_width as u32, lines, data);
    let map = Map::new(grid, start, end);

    Ok(map)
}

#[derive(Debug, Clone)]
struct Map {
    grid: Grid<Tile>,
    path: Vec<Coordinates>,
    start: Coordinates,
    #[allow(unused)]
    end: Coordinates,
}

impl Map {
    pub fn new(grid: Grid<Tile>, start: Coordinates, end: Coordinates) -> Self {
        let mut new = Self {
            grid,
            path: Vec::new(),
            start,
            end,
        };

        new.flood_fill_path();
        new
    }

    fn flood_fill_path(&mut self) {
        let mut stack = Vec::new();
        stack.push((self.start, 0));

        while let Some((node, distance)) = stack.pop() {
            match &mut self.grid[node] {
                Tile::Path(d) if *d >= 0 => continue,
                Tile::Path(d) => {
                    *d = distance;
                    self.path.push(node);
                }
                _ => unreachable!(),
            }

            for neighbor in CardinalNeighbors::new(node) {
                if self
                    .grid
                    .get(neighbor)
                    .is_some_and(|tile| matches!(tile, Tile::Path(d) if *d < 0))
                {
                    stack.push((neighbor, distance + 1))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path(i64),
    Wall,
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
