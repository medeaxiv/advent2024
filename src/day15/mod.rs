use std::collections::VecDeque;

use allocator_api2::vec::Vec;
use aoc_utils::{
    direction::{Direction, Orientation},
    display::DisplayChar,
    grid::{Coordinates, Grid},
    hashbrown::HashSet,
    str::StrExt,
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

fn part_1(input: &str) -> anyhow::Result<u64> {
    let (mut map, mut robot, path) = parse(input)?;

    for &direction in path.iter() {
        move_robot(&mut robot, &mut map, direction);
    }

    let result = gps_sum(&map.crates);
    Ok(result)
}

fn move_robot(robot: &mut Coordinates, map: &mut Map, direction: Direction) {
    let next_position = direction.apply_movement(*robot, 1);

    let mut is_pushing_crate = false;
    let mut push_cursor = next_position;
    while matches!(map.grid[push_cursor], Tile::Crate(..)) {
        is_pushing_crate = true;
        push_cursor = direction.apply_movement(push_cursor, 1);
    }

    if map.grid[push_cursor] == Tile::Empty {
        if is_pushing_crate {
            map.swap(next_position, push_cursor);
        }

        *robot = next_position;
    }
}

fn gps_sum(crates: &[Coordinates]) -> u64 {
    crates
        .into_iter()
        .map(|coordinates| coordinates.x as u64 + 100 * coordinates.y as u64)
        .sum()
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let (map, mut robot, path) = parse(input)?;
    let mut map = WideMap::new(map);
    robot.x *= 2;

    for &direction in path.iter() {
        wide_move_robot(&mut robot, &mut map, direction);
    }

    let result = gps_sum(&map.crates);
    Ok(result)
}

fn wide_move_robot(robot: &mut Coordinates, map: &mut WideMap, direction: Direction) {
    let next_position = direction.apply_movement(*robot, 1);

    let allow_movement = match map.grid[next_position] {
        WideTile::Empty => true,
        WideTile::Wall => false,
        WideTile::CrateLeft(idx) | WideTile::CrateRight(idx) => map.try_push_crate(idx, direction),
    };

    if allow_movement {
        *robot = next_position;
    }
}

#[allow(dead_code)]
fn print_map<T: DisplayChar>(grid: &Grid<T>, robot: Coordinates) {
    let mut map = String::with_capacity(grid.len());
    for (y, row) in grid.rows().enumerate() {
        if y != 0 {
            map.push_str("\r\n");
        }

        for (x, tile) in row.iter().enumerate() {
            if x == robot.x as usize && y == robot.y as usize {
                map.push('@');
            } else {
                map.push(tile.display_char());
            }
        }
    }

    println!("{map}");
}

fn parse(input: &str) -> anyhow::Result<(Map, Coordinates, Vec<Direction>)> {
    let mut paragraphs = input.paragraphs();
    let (map, robot) = parse_map(paragraphs.next().ok_or(AocError::InvalidInput)?)?;
    let path = parse_path(paragraphs.next().ok_or(AocError::InvalidInput)?)?;
    Ok((map, robot, path))
}

fn parse_map(input: &str) -> anyhow::Result<(Map, Coordinates)> {
    let line_length = input
        .lines()
        .next()
        .ok_or(AocError::InvalidInput)?
        .chars()
        .count();

    let mut lines = 0;
    let mut robot = Coordinates::zeros();
    let mut data = Vec::new();
    let mut crates = Vec::new();
    for (y, line) in input.lines().enumerate() {
        lines += 1;

        for (x, c) in line.chars().enumerate() {
            let coordinates = Coordinates::new(x as u32, y as u32);

            match c {
                '.' => data.push(Tile::Empty),
                'O' => {
                    data.push(Tile::Crate(crates.len() as u32));
                    crates.push(coordinates);
                }
                '#' => data.push(Tile::Wall),
                '@' => {
                    data.push(Tile::Empty);
                    robot = coordinates;
                }
                _ => return Err(AocError::InvalidInput.into()),
            }
        }
    }

    let grid = Grid::from_vec(line_length as u32, lines, data);
    let map = Map { grid, crates };
    Ok((map, robot))
}

fn parse_path(input: &str) -> anyhow::Result<Vec<Direction>> {
    let mut path = Vec::new();
    for c in input.chars() {
        let direction = match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => continue,
        };
        path.push(direction);
    }

    Ok(path)
}

struct Map {
    grid: Grid<Tile>,
    crates: Vec<Coordinates>,
}

impl Map {
    pub fn swap(&mut self, a: Coordinates, b: Coordinates) {
        let mut closure = || {
            let a_index = self.grid.get_index(a)?;
            let b_index = self.grid.get_index(b)?;

            if let Tile::Crate(idx) = self.grid[a_index] {
                self.crates[idx as usize] = b;
            }

            if let Tile::Crate(idx) = self.grid[b_index] {
                self.crates[idx as usize] = a;
            }

            self.grid.as_mut_slice().swap(a_index, b_index);

            Some(())
        };

        closure();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Crate(u32),
    Wall,
}

impl DisplayChar for Tile {
    fn display_char(&self) -> char {
        match self {
            Self::Empty => ' ',
            Self::Crate(..) => 'O',
            Self::Wall => '#',
        }
    }
}

struct WideMap {
    grid: Grid<WideTile>,
    crates: Vec<Coordinates>,
}

impl WideMap {
    pub fn new(map: Map) -> Self {
        let width = map.grid.width() * 2;
        let height = map.grid.height();
        let mut data = Vec::with_capacity(map.grid.len() * 2);

        data.extend(map.grid.into_iter().flat_map(|tile| match tile {
            Tile::Empty => [WideTile::Empty, WideTile::Empty],
            Tile::Crate(idx) => [WideTile::CrateLeft(idx), WideTile::CrateRight(idx)],
            Tile::Wall => [WideTile::Wall, WideTile::Wall],
        }));

        let grid = Grid::from_vec(width, height, data);
        let mut new = Self {
            grid,
            crates: map.crates,
        };

        for c in new.crates.iter_mut() {
            c.x *= 2;
        }

        new
    }

    pub fn try_push_crate(&mut self, idx: u32, direction: Direction) -> bool {
        match direction.axis() {
            Orientation::Horizontal => self.try_push_crate_horizontal(idx, direction),
            Orientation::Vertical => self.try_push_crate_vertical(idx, direction),
        }
    }

    fn try_push_crate_horizontal(&mut self, idx: u32, direction: Direction) -> bool {
        let Some(&left) = self.crates.get(idx as usize) else {
            return false;
        };

        let right = Coordinates::new(left.x + 1, left.y);

        let (leading, trailing) = if direction.is_positive() {
            (right, left)
        } else {
            (left, right)
        };

        let next = direction.apply_movement(leading, 1);

        let push = match self.grid[next] {
            WideTile::Empty => true,
            WideTile::Wall => return false,
            WideTile::CrateLeft(next_idx) | WideTile::CrateRight(next_idx) => {
                self.try_push_crate_horizontal(next_idx, direction)
            }
        };

        if push {
            self.grid.swap(leading, next);
            self.grid.swap(trailing, leading);
            self.crates[idx as usize] = direction.apply_movement(left, 1);
        }

        push
    }

    fn try_push_crate_vertical(&mut self, idx: u32, direction: Direction) -> bool {
        let mut moves = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(idx);

        while let Some(node) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }

            let (left, right, next_left, next_right) =
                if let Some(&left) = self.crates.get(node as usize) {
                    let next_left = direction.apply_movement(left, 1);
                    let right = Coordinates::new(left.x + 1, left.y);
                    let next_right = Coordinates::new(next_left.x + 1, next_left.y);
                    (left, right, next_left, next_right)
                } else {
                    continue;
                };

            moves.push((node, left, right, next_left, next_right));

            match self.grid[next_left] {
                WideTile::Wall => return false,
                WideTile::CrateLeft(next) => queue.push_back(next),
                WideTile::CrateRight(next) => queue.push_back(next),
                WideTile::Empty => {}
            }

            match self.grid[next_right] {
                WideTile::Wall => return false,
                WideTile::CrateLeft(next) => queue.push_back(next),
                WideTile::CrateRight(next) => queue.push_back(next),
                WideTile::Empty => {}
            }
        }

        for &(node, left, right, next_left, next_right) in moves.iter().rev() {
            self.grid.swap(left, next_left);
            self.grid.swap(right, next_right);
            self.crates[node as usize] = next_left;
        }

        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WideTile {
    Empty,
    CrateLeft(u32),
    CrateRight(u32),
    Wall,
}

impl DisplayChar for WideTile {
    fn display_char(&self) -> char {
        match self {
            Self::Empty => ' ',
            Self::CrateLeft(..) => '[',
            Self::CrateRight(..) => ']',
            Self::Wall => '#',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");
    const EXAMPLE_2: &str = include_str!("example.2.txt");
    const EXAMPLE_3: &str = include_str!("example.3.txt");

    #[rstest]
    #[case(EXAMPLE_1, 2028)]
    #[case(EXAMPLE_2, 10092)]
    fn test_part_1(#[case] input: &str, #[case] expected: u64) {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(EXAMPLE_3, 618)]
    #[case(EXAMPLE_2, 9021)]
    fn test_part_2(#[case] input: &str, #[case] expected: u64) {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_2(input).unwrap();
        assert_eq!(result, expected);
    }
}
