use allocator_api2::vec::Vec;
use aoc_utils::{
    grid::{Coordinates, Grid},
    hashbrown::HashSet,
    neighbors::CardinalNeighbors,
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
    let map = parse(input)?;
    let total_price = map
        .regions
        .iter()
        .map(|region| region.area * region.perimeter)
        .sum();
    Ok(total_price)
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let map = parse(input)?;
    let total_price = map
        .regions
        .iter()
        .map(|region| region.area * region.corners)
        .sum();
    Ok(total_price)
}

fn parse(input: &str) -> anyhow::Result<Map> {
    let line_length = input
        .lines()
        .next()
        .ok_or(AocError::InvalidInput)?
        .chars()
        .count();

    let mut line_count = 0;
    let mut data = Vec::with_capacity(line_length * line_length);
    for line in input.lines() {
        line_count += 1;

        let tiles = line.chars().map(Tile::new);
        data.extend(tiles);
    }

    let grid = Grid::from_vec(line_length as u32, line_count, data);
    let mut map = Map {
        grid,
        regions: Vec::new(),
    };

    map.compute_regions();

    Ok(map)
}

#[derive(Debug, Clone)]
struct Map {
    grid: Grid<Tile>,
    regions: Vec<Region>,
}

impl Map {
    pub fn compute_regions(&mut self) {
        for i in 0..self.grid.len() {
            self.compute_region(i);
        }

        for y in 0..=self.grid.height() {
            self.count_corners(y);
        }
    }

    fn compute_region(&mut self, index: usize) {
        let Some(origin) = self.grid.get_at_index(index).copied() else {
            return;
        };

        if origin.region.is_some() {
            return;
        }

        let Some(coordinates) = self.grid.get_coordinates(index) else {
            return;
        };

        let region_index = self.regions.len();
        let mut region = Region::default();

        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(coordinates);
        while let Some(coordinates) = stack.pop() {
            if !visited.insert(coordinates) {
                continue;
            }

            self.grid[coordinates].region = Some(region_index);

            let mut neighbors = 0;
            for candidate in CardinalNeighbors::new(coordinates) {
                let Some(tile) = self.grid.get(candidate) else {
                    continue;
                };

                if tile.plant == origin.plant {
                    stack.push(candidate);
                    neighbors += 1;
                }
            }

            region.area += 1;
            region.perimeter += 4 - neighbors;
        }

        self.regions.push(region);
    }

    fn count_corners(&mut self, y: u32) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum State {
            None,
            Same(usize),
            Different(usize, usize),
            Down(usize),
            Up(usize),
        }

        impl State {
            fn new(up: Option<usize>, down: Option<usize>) -> Self {
                match (up, down) {
                    (None, None) => Self::None,
                    (Some(up), None) => Self::Up(up),
                    (None, Some(down)) => Self::Down(down),
                    (Some(up), Some(down)) if up == down => Self::Same(up),
                    (Some(up), Some(down)) => Self::Different(up, down),
                }
            }
        }

        let mut state = State::None;

        for x in 0..=self.grid.width() {
            let up = y.checked_sub(1).map(|y| Coordinates::new(x, y));
            let down = Coordinates::new(x, y);
            let next_state = State::new(
                up.and_then(|coordinates| self.grid.get(coordinates))
                    .and_then(|tile| tile.region),
                self.grid.get(down).and_then(|tile| tile.region),
            );

            if next_state == state {
                continue;
            }

            match state {
                State::None => match next_state {
                    State::Down(down) => self.regions[down].corners += 1,
                    State::Up(up) => self.regions[up].corners += 1,
                    State::Different(up, down) => {
                        self.regions[up].corners += 1;
                        self.regions[down].corners += 1;
                    }
                    _ => {}
                },
                State::Same(..) => match next_state {
                    State::Different(up, down) => {
                        self.regions[up].corners += 1;
                        self.regions[down].corners += 1;
                    }
                    _ => {}
                },
                State::Different(previous_up, previous_down) => match next_state {
                    State::None | State::Same(..) => {
                        self.regions[previous_up].corners += 1;
                        self.regions[previous_down].corners += 1;
                    }
                    State::Different(up, down) if up != previous_up && down != previous_down => {
                        self.regions[previous_up].corners += 1;
                        self.regions[previous_down].corners += 1;
                        self.regions[up].corners += 1;
                        self.regions[down].corners += 1;
                    }
                    State::Different(up, down) if up != previous_up && down == previous_down => {
                        self.regions[previous_up].corners += 1;
                        self.regions[up].corners += 1;
                    }
                    State::Different(up, down) if up == previous_up && down != previous_down => {
                        self.regions[previous_down].corners += 1;
                        self.regions[down].corners += 1;
                    }
                    _ => {}
                },
                State::Up(previous_up) => match next_state {
                    State::Up(up) if up != previous_up => {
                        self.regions[previous_up].corners += 1;
                        self.regions[up].corners += 1;
                    }
                    State::None => self.regions[previous_up].corners += 1,
                    _ => {}
                },
                State::Down(previous_down) => match next_state {
                    State::Down(down) if down != previous_down => {
                        self.regions[previous_down].corners += 1;
                        self.regions[down].corners += 1;
                    }
                    State::None => self.regions[previous_down].corners += 1,
                    _ => {}
                },
            }

            state = next_state;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    plant: char,
    region: Option<usize>,
}

impl Tile {
    pub fn new(plant: char) -> Self {
        Self {
            plant,
            region: None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Region {
    area: u64,
    perimeter: u64,
    corners: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");
    const EXAMPLE_2: &str = include_str!("example.2.txt");
    const EXAMPLE_3: &str = include_str!("example.3.txt");
    const EXAMPLE_4: &str = include_str!("example.4.txt");

    #[rstest]
    #[case(EXAMPLE_1, 772)]
    #[case(EXAMPLE_2, 1930)]
    fn test_part_1(#[case] input: &str, #[case] expected: u64) {
        let result = part_1(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(EXAMPLE_1, 436)]
    #[case(EXAMPLE_2, 1206)]
    #[case(EXAMPLE_3, 236)]
    #[case(EXAMPLE_4, 368)]
    fn test_part_2(#[case] input: &str, #[case] expected: u64) {
        let result = part_2(input).unwrap();
        assert_eq!(result, expected);
    }
}
