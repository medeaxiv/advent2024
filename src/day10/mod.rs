use allocator_api2::vec::Vec;
use aoc_utils::{
    grid::{Coordinates, Grid},
    neighbors::CardinalNeighbors,
    search::breadth_first_search,
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
    let scores = trail_scores_by_peaks(&map);
    let total_score = map
        .trailheads
        .iter()
        .map(|&coordinates| scores[coordinates])
        .sum::<u64>();
    Ok(total_score)
}

fn trail_scores_by_peaks(map: &Map) -> Grid<u64> {
    let mut scores = Grid::new(map.grid.width(), map.grid.height());

    for trailpeak in map.trailpeaks.iter() {
        let _: Option<()> = breadth_first_search(
            trailpeak,
            |&coordinates| {
                let height = map.grid[coordinates];
                CardinalNeighbors::new(coordinates).filter(move |neighbor| {
                    map.grid
                        .get(*neighbor)
                        .is_some_and(|neighbor_height| *neighbor_height + 1 == height)
                })
            },
            |coordinates| {
                scores[*coordinates] += 1;
                None
            },
        );
    }

    scores
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let map = parse(input)?;
    let scores = trail_scores_by_paths(&map);
    let total_score = map
        .trailheads
        .iter()
        .map(|&coordinates| scores[coordinates])
        .sum::<u64>();
    Ok(total_score)
}

fn trail_scores_by_paths(map: &Map) -> Grid<u64> {
    fn recurse(coordinates: Coordinates, map: &Map, scores: &mut Grid<u64>) -> u64 {
        let cached_score = scores[coordinates];
        if cached_score != 0 {
            return cached_score;
        }

        let height = map.grid[coordinates];
        let next_height = height + 1;

        let mut score = 0;
        for neighbor in CardinalNeighbors::new(coordinates) {
            let Some(&neighbor_height) = map.grid.get(neighbor) else {
                continue;
            };

            if neighbor_height != next_height {
                continue;
            }

            score += recurse(neighbor, map, scores);
        }

        scores[coordinates] = score;
        score
    }

    let mut scores = Grid::new(map.grid.width(), map.grid.height());

    for &trailpeak in map.trailpeaks.iter() {
        scores[trailpeak] = 1;
    }

    for &trailhead in map.trailheads.iter() {
        recurse(trailhead, map, &mut scores);
    }

    scores
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
