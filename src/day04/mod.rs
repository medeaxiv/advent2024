use allocator_api2::vec::Vec;
use aoc_utils::{
    grid::{Coordinates, Grid},
    numerics::min_max,
    AocError,
};

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    let grid = parse(input)?;

    let mut count = 0;
    for index in 0..grid.len() {
        let coordinates = grid.get_coordinates(index).expect("Index should be valid");
        count += count_word_instances(&grid, coordinates, "XMAS");
    }

    Ok(count)
}

fn count_word_instances(grid: &Grid<char>, start_coordinates: Coordinates, word: &str) -> i64 {
    let (head, tail) = {
        let mut chars = word.chars();
        let head = chars.next();
        (head, chars.as_str())
    };

    let start = grid.get(start_coordinates).copied();
    if start.is_none() || start != head {
        return 0;
    }

    Direction::ALL
        .into_iter()
        .filter_map(|direction| {
            direction
                .offset(start_coordinates)
                .map(|coordinates| (direction, coordinates))
        })
        .map(|(direction, coordinates)| is_word_instance(grid, coordinates, tail, direction) as i64)
        .sum()
}

fn is_word_instance(
    grid: &Grid<char>,
    start_coordinates: Coordinates,
    word: &str,
    direction: Direction,
) -> bool {
    let mut coordinates = start_coordinates;
    for (index, expected) in word.chars().enumerate() {
        if index != 0 {
            if let Some(next) = direction.offset(coordinates) {
                coordinates = next;
            } else {
                return false;
            }
        }

        if let Some(&actual) = grid.get(coordinates) {
            if expected != actual {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    let grid = parse(input)?;

    let mut count = 0;
    for index in 0..grid.len() {
        let coordinates = grid.get_coordinates(index).expect("Index should be valid");
        count += is_x_mas(&grid, coordinates) as i64;
    }

    Ok(count)
}

fn is_x_mas(grid: &Grid<char>, coordinates: Coordinates) -> bool {
    let center = grid.get(coordinates);
    if !matches!(center, Some('A')) {
        return false;
    }

    let neighbors = Direction::INTERCARDINAL.map(|direction| {
        direction
            .offset(coordinates)
            .and_then(|coordinates| grid.get(coordinates).copied())
    });

    let down = min_max(neighbors[0], neighbors[2]);
    let up = min_max(neighbors[1], neighbors[3]);
    down == (Some('M'), Some('S')) && up == (Some('M'), Some('S'))
}

fn parse(input: &str) -> Result<Grid<char>, AocError> {
    let line_length = input
        .lines()
        .next()
        .ok_or(AocError::InvalidInput)?
        .chars()
        .count();

    // Assume grid to be a square
    let mut data = Vec::with_capacity(line_length * line_length);
    let mut lines = 0;
    for line in input.lines() {
        lines += 1;
        data.extend(line.chars());
    }

    let grid = Grid::from_vec(line_length as u32, lines, data);
    Ok(grid)
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    const ALL: [Self; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];

    const INTERCARDINAL: [Self; 4] = [
        Self::NorthEast,
        Self::SouthEast,
        Self::SouthWest,
        Self::NorthWest,
    ];

    pub fn offset(&self, coordinates: Coordinates) -> Option<Coordinates> {
        match self {
            Self::North => {
                let x = coordinates.x;
                let y = coordinates.y.checked_sub(1)?;
                Some(Coordinates::new(x, y))
            }
            Self::NorthEast => {
                let x = coordinates.x.checked_add(1)?;
                let y = coordinates.y.checked_sub(1)?;
                Some(Coordinates::new(x, y))
            }
            Self::East => {
                let x = coordinates.x.checked_add(1)?;
                let y = coordinates.y;
                Some(Coordinates::new(x, y))
            }
            Self::SouthEast => {
                let x = coordinates.x.checked_add(1)?;
                let y = coordinates.y.checked_add(1)?;
                Some(Coordinates::new(x, y))
            }
            Self::South => {
                let x = coordinates.x;
                let y = coordinates.y.checked_add(1)?;
                Some(Coordinates::new(x, y))
            }
            Self::SouthWest => {
                let x = coordinates.x.checked_sub(1)?;
                let y = coordinates.y.checked_add(1)?;
                Some(Coordinates::new(x, y))
            }
            Self::West => {
                let x = coordinates.x.checked_sub(1)?;
                let y = coordinates.y;
                Some(Coordinates::new(x, y))
            }
            Self::NorthWest => {
                let x = coordinates.x.checked_sub(1)?;
                let y = coordinates.y.checked_sub(1)?;
                Some(Coordinates::new(x, y))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE_1).unwrap();
        assert_eq!(result, 9)
    }
}
