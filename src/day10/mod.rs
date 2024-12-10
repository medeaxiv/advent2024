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
    let scores = trail_scores(&map);
    let total_score = map
        .trailheads
        .iter()
        .map(|&coordinates| scores[coordinates])
        .sum::<u64>();
    Ok(total_score)
}

fn trail_scores(map: &Map) -> Grid<u64> {
    let mut scores = Grid::new(map.grid.width(), map.grid.height());

    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    for &trailpeak in map.trailpeaks.iter() {
        stack.push(trailpeak);

        while let Some(node) = stack.pop() {
            if !visited.insert(node) {
                continue;
            }

            let height = map.grid[node];

            scores[node] += 1;

            let neighbors = CardinalNeighbors::new(node).filter(move |neighbor| {
                map.grid
                    .get(*neighbor)
                    .is_some_and(|neighbor_height| *neighbor_height + 1 == height)
            });

            stack.extend(neighbors);
        }

        visited.clear();
    }

    scores
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let map = parse(input)?;
    let ratings = trail_ratings(&map);
    let total_rating = map
        .trailheads
        .iter()
        .map(|&coordinates| ratings[coordinates])
        .sum::<u64>();
    Ok(total_rating)
}

fn trail_ratings(map: &Map) -> Grid<u64> {
    fn recurse(coordinates: Coordinates, map: &Map, ratings: &mut Grid<u64>) -> u64 {
        let cached_rating = ratings[coordinates];
        if cached_rating != 0 {
            return cached_rating;
        }

        let height = map.grid[coordinates];
        let next_height = height + 1;

        let mut rating = 0;
        for neighbor in CardinalNeighbors::new(coordinates) {
            let Some(&neighbor_height) = map.grid.get(neighbor) else {
                continue;
            };

            if neighbor_height != next_height {
                continue;
            }

            rating += recurse(neighbor, map, ratings);
        }

        ratings[coordinates] = rating;
        rating
    }

    let mut ratings = Grid::new(map.grid.width(), map.grid.height());

    for &trailpeak in map.trailpeaks.iter() {
        ratings[trailpeak] = 1;
    }

    for &trailhead in map.trailheads.iter() {
        recurse(trailhead, map, &mut ratings);
    }

    ratings
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
    let mut trailheads = Vec::new();
    let mut trailpeaks = Vec::new();
    for (y, line) in input.lines().enumerate() {
        line_count += 1;

        for (x, c) in line.chars().enumerate() {
            let height = match c {
                '.' => 100, // something unreachable
                c => c.to_digit(10).ok_or(AocError::InvalidInput)? as u8,
            };

            data.push(height);

            match height {
                0 => trailheads.push(Coordinates::new(x as u32, y as u32)),
                9 => trailpeaks.push(Coordinates::new(x as u32, y as u32)),
                _ => {}
            }
        }
    }

    let grid = Grid::from_vec(line_length as u32, line_count, data);
    let map = Map {
        grid,
        trailheads,
        trailpeaks,
    };

    Ok(map)
}

#[derive(Debug)]
struct Map {
    grid: Grid<u8>,
    trailheads: Vec<Coordinates>,
    trailpeaks: Vec<Coordinates>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");
    const EXAMPLE_2: &str = include_str!("example.2.txt");
    const EXAMPLE_3: &str = include_str!("example.3.txt");
    const EXAMPLE_4: &str = include_str!("example.4.txt");
    const EXAMPLE_5: &str = include_str!("example.5.txt");
    const EXAMPLE_6: &str = include_str!("example.6.txt");
    const EXAMPLE_7: &str = include_str!("example.7.txt");

    #[rstest]
    #[case(EXAMPLE_1, 1)]
    #[case(EXAMPLE_2, 36)]
    #[case(EXAMPLE_3, 2)]
    #[case(EXAMPLE_4, 4)]
    #[case(EXAMPLE_5, 3)]
    fn test_part_1(#[case] input: &str, #[case] expected: u64) {
        let result = part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(EXAMPLE_1, 16)]
    #[case(EXAMPLE_2, 81)]
    #[case(EXAMPLE_6, 3)]
    #[case(EXAMPLE_4, 13)]
    #[case(EXAMPLE_7, 227)]
    fn test_part_2(#[case] input: &str, #[case] expected: u64) {
        let result = part_2(input).unwrap();
        assert_eq!(result, expected)
    }
}
