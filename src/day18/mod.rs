use std::collections::VecDeque;

use aoc_utils::{
    grid::{Coordinates, Grid},
    hashbrown::{hash_map::Entry, HashMap},
    neighbors::CardinalNeighbors,
    parser::VectorParseExt,
    AocError,
};

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT, 1024, 71, 71));
    builder.add_part(|| part_2(INPUT, 71, 71));
}

fn part_1(input: &str, time: u32, width: u32, height: u32) -> anyhow::Result<u32> {
    let (grid, _) = parse(input, width, height)?;
    find_path_length(&grid, time).ok_or(AocError::message("Unable to find path").into())
}

fn find_path_length(grid: &Grid<u32>, time: u32) -> Option<u32> {
    let target = Coordinates::new(grid.width() - 1, grid.height() - 1);

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((Coordinates::zeros(), 0));

    while let Some((node, step)) = queue.pop_front() {
        match visited.entry(node) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(vacant) => {
                vacant.insert(step);
            }
        }

        if node == target {
            return Some(step);
        }

        for neighbor in CardinalNeighbors::new(node) {
            if grid
                .get(neighbor)
                .is_some_and(|&byte_index| byte_index >= time)
            {
                queue.push_back((neighbor, step + 1));
            }
        }
    }

    None
}

fn part_2(input: &str, width: u32, height: u32) -> anyhow::Result<String> {
    let (grid, bytes) = parse(input, width, height)?;
    find_first_byte_blocking_path(&grid, &bytes)
        .map(|byte| format!("{},{}", byte.x, byte.y))
        .ok_or(AocError::message("Path is never blocked").into())
}

fn find_first_byte_blocking_path(grid: &Grid<u32>, bytes: &[Coordinates]) -> Option<Coordinates> {
    let mut lower = 0;
    let mut upper = bytes.len() as u32;
    let mut byte_index;
    let mut result = None;

    while lower < upper {
        byte_index = lower + (upper - lower) / 2;
        let time = byte_index + 1;

        let can_find_path = find_path_length(&grid, time).is_some();
        if can_find_path {
            lower = byte_index + 1;
        } else {
            result = Some(bytes[byte_index as usize]);
            upper = byte_index - 1;
        }
    }

    result
}

fn parse(input: &str, width: u32, height: u32) -> anyhow::Result<(Grid<u32>, Vec<Coordinates>)> {
    let mut bytes = Vec::new();
    let mut grid = Grid::new_with(width, height, || u32::MAX);
    for (index, line) in input.lines().enumerate() {
        let coordinates = Coordinates::from_str_radix(line, 10)?;
        bytes.push(coordinates);
        grid[coordinates] = index as u32;
    }

    Ok((grid, bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_1(EXAMPLE_1, 12, 7, 7).unwrap();
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_2() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_2(EXAMPLE_1, 7, 7).unwrap();
        assert_eq!(result, "6,1");
    }
}
