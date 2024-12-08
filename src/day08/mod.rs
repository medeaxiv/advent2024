use aoc_utils::{
    hashbrown::{HashMap, HashSet},
    nalgebra,
};
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let map = parse(input)?;

    let mut antinodes = HashSet::new();
    for frequency in map.frequencies_iter() {
        for (&a, &b) in map.antennas(frequency).iter().tuple_combinations() {
            let delta = b - a;

            let anti_a = a - delta;
            if map.is_in_bounds(anti_a) {
                antinodes.insert(anti_a);
            }

            let anti_b = b + delta;
            if map.is_in_bounds(anti_b) {
                antinodes.insert(anti_b);
            }
        }
    }

    Ok(antinodes.len())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let map = parse(input)?;

    let mut antinodes = HashSet::new();
    for frequency in map.frequencies_iter() {
        for (&a, &b) in map.antennas(frequency).iter().tuple_combinations() {
            let delta = b - a;

            let mut anti_a = a;
            while map.is_in_bounds(anti_a) {
                antinodes.insert(anti_a);
                anti_a -= delta;
            }

            let mut anti_b = b;
            while map.is_in_bounds(anti_b) {
                antinodes.insert(anti_b);
                anti_b += delta;
            }
        }
    }

    Ok(antinodes.len())
}

fn parse(input: &str) -> anyhow::Result<Map> {
    let mut width = 0;
    let mut height = 0;
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        if y >= height {
            height = y + 1;
        }

        for (x, c) in line.chars().enumerate() {
            if x >= width {
                width = x + 1;
            }

            match c {
                '.' => {}
                c => map.insert(c, Coordinates::new(x as i32, y as i32)),
            }
        }
    }

    map.set_size(width as i32, height as i32);

    Ok(map)
}

type Coordinates = nalgebra::Vector2<i32>;

struct Map {
    antennas: HashMap<char, Vec<Coordinates>>,
    size: Coordinates,
}

impl Map {
    pub fn new() -> Self {
        Self {
            antennas: HashMap::new(),
            size: Coordinates::zeros(),
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.size.x = width.abs();
        self.size.y = height.abs();
    }

    pub fn insert(&mut self, frequency: char, coordinates: Coordinates) {
        let entry = self.antennas.entry(frequency).or_default();
        entry.push(coordinates);
    }

    pub fn frequencies_iter(&self) -> impl Iterator<Item = char> + use<'_> {
        self.antennas.keys().copied()
    }

    pub fn antennas(&self, frequency: char) -> &[Coordinates] {
        if let Some(antennas) = self.antennas.get(&frequency) {
            antennas.as_slice()
        } else {
            &[]
        }
    }

    pub fn is_in_bounds(&self, coordinates: Coordinates) -> bool {
        (0..self.size.x).contains(&coordinates.x) && (0..self.size.y).contains(&coordinates.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE_1).unwrap();
        assert_eq!(result, 34);
    }
}
