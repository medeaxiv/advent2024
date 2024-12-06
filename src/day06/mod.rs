use aoc_utils::{
    direction::{Direction, Orientation},
    grid::Coordinates,
    hashbrown::{HashMap, HashSet},
    AocError,
};
use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let (start_coordinates, lab) = parse(input)?;
    let visited = visit_path(&lab, start_coordinates);

    Ok(visited.len())
}

fn visit_path(lab: &Lab, start_coordinates: Coordinates) -> HashSet<Coordinates> {
    let mut guard = Guard {
        position: start_coordinates,
        direction: Direction::Up,
    };

    let mut visited = HashSet::new();
    visited.insert(guard.position);

    loop {
        let result = step(lab, &guard);
        let distance = result.distance();

        for _ in 0..distance {
            guard.apply_movement(1);
            visited.insert(guard.position);
        }

        match result {
            StepResult::Obstacle(_) => {
                guard.turn();
            }
            StepResult::Exited(_) => {
                break;
            }
        }
    }

    visited
}

fn step(lab: &Lab, guard: &Guard) -> StepResult {
    match lab.find_obstacle(guard.position, guard.direction) {
        Ok((_, dist)) => StepResult::Obstacle(dist),
        Err(FindObstacleError::NoObstacle(dist)) => StepResult::Exited(dist),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy)]
enum StepResult {
    Obstacle(u32),
    Exited(u32),
}

impl StepResult {
    pub fn distance(&self) -> u32 {
        match *self {
            Self::Obstacle(dist) => dist,
            Self::Exited(dist) => dist,
        }
    }
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let (start_coordinates, lab) = parse(input)?;
    let mut candidates = visit_path(&lab, start_coordinates);
    candidates.remove(&start_coordinates);
    let candidates = Vec::from_iter(candidates);

    let count = candidates
        .par_iter()
        .filter(|&&candidate| {
            let mut lab = lab.clone();
            lab.insert(candidate);
            find_loop(&lab, start_coordinates) == FindLoopResult::Loop
        })
        .count();

    Ok(count)
}

fn find_loop(lab: &Lab, start_coordinates: Coordinates) -> FindLoopResult {
    let mut guard = Guard {
        position: start_coordinates,
        direction: Direction::Up,
    };

    let mut visited = HashSet::new();

    loop {
        let distance = match step(lab, &guard) {
            StepResult::Exited(_) => return FindLoopResult::Exited,
            StepResult::Obstacle(dist) => dist,
        };

        guard.apply_movement(distance);
        guard.turn();

        if !visited.insert((guard.position, guard.direction)) {
            return FindLoopResult::Loop;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FindLoopResult {
    Loop,
    Exited,
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    position: Coordinates,
    direction: Direction,
}

impl Guard {
    pub fn apply_movement(&mut self, distance: u32) {
        self.position = self.direction.apply_movement(self.position, distance);
    }

    pub fn turn(&mut self) {
        self.direction = self.direction.turn_right();
    }
}

fn parse(input: &str) -> anyhow::Result<(Coordinates, Lab)> {
    let mut start_coordinates = None;
    let mut width = 0;
    let mut height = 0;
    let mut lab = Lab::new();
    for (y, line) in input.lines().enumerate() {
        if y >= height {
            height = y + 1;
        }

        for (x, c) in line.chars().enumerate() {
            if x >= width {
                width = x + 1;
            }

            match c {
                '#' => lab.insert(Coordinates::new(x as u32, y as u32)),
                '^' => start_coordinates = Some(Coordinates::new(x as u32, y as u32)),
                _ => {}
            }
        }
    }

    lab.set_size(width as u32, height as u32);

    let start_coordinates = start_coordinates.ok_or(AocError::InvalidInput)?;
    Ok((start_coordinates, lab))
}

#[derive(Debug, Clone)]
struct Lab {
    size: Coordinates,
    rows: HashMap<u32, Vec<u32>>,
    columns: HashMap<u32, Vec<u32>>,
}

impl Lab {
    pub fn new() -> Self {
        Self {
            size: Coordinates::zeros(),
            rows: HashMap::new(),
            columns: HashMap::new(),
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size.x = width;
        self.size.y = height;
    }

    pub fn insert(&mut self, coordinates: Coordinates) {
        Self::insert_in(&mut self.rows, coordinates.y, coordinates.x);
        Self::insert_in(&mut self.columns, coordinates.x, coordinates.y);
    }

    pub fn find_obstacle(
        &self,
        from: Coordinates,
        direction: Direction,
    ) -> Result<(Coordinates, u32), FindObstacleError> {
        match direction.axis() {
            Orientation::Horizontal => self.find_obstacle_horizontal(from, direction),
            Orientation::Vertical => self.find_obstacle_vertical(from, direction),
        }
    }

    fn find_obstacle_horizontal(
        &self,
        from: Coordinates,
        direction: Direction,
    ) -> Result<(Coordinates, u32), FindObstacleError> {
        let (previous, next) = Self::find_surrounding_in(&self.rows, from.y, from.x)?;

        let major = if direction.is_positive() {
            next.ok_or_else(|| {
                let dist = self.size.x - from.x - 1;
                FindObstacleError::NoObstacle(dist)
            })?
        } else {
            previous.ok_or_else(|| {
                let dist = from.x;
                FindObstacleError::NoObstacle(dist)
            })?
        };

        let obstacle = Coordinates::new(major, from.y);
        let dist = from.x.abs_diff(major) - 1;
        Ok((obstacle, dist))
    }

    fn find_obstacle_vertical(
        &self,
        from: Coordinates,
        direction: Direction,
    ) -> Result<(Coordinates, u32), FindObstacleError> {
        let (previous, next) = Self::find_surrounding_in(&self.columns, from.x, from.y)?;

        let major = if direction.is_positive() {
            next.ok_or_else(|| {
                let dist = self.size.y - from.y - 1;
                FindObstacleError::NoObstacle(dist)
            })?
        } else {
            previous.ok_or_else(|| {
                let dist = from.y;
                FindObstacleError::NoObstacle(dist)
            })?
        };

        let obstacle = Coordinates::new(from.x, major);
        let dist = from.y.abs_diff(major) - 1;
        Ok((obstacle, dist))
    }

    fn insert_in(map: &mut HashMap<u32, Vec<u32>>, key: u32, value: u32) {
        let entry = map.entry(key).or_default();
        match entry.binary_search(&value) {
            Ok(_) => {}
            Err(index) => {
                entry.insert(index, value);
            }
        }
    }

    fn find_surrounding_in(
        map: &HashMap<u32, Vec<u32>>,
        key: u32,
        from: u32,
    ) -> Result<(Option<u32>, Option<u32>), FindObstacleError> {
        if let Some(entry) = map.get(&key) {
            match entry.binary_search(&from) {
                Ok(_) => Err(FindObstacleError::OnObstacle),
                Err(index) => {
                    let previous = index.checked_sub(1).map(|index| entry[index]);
                    let next = entry.get(index).copied();
                    Ok((previous, next))
                }
            }
        } else {
            Ok((None, None))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FindObstacleError {
    OnObstacle,
    NoObstacle(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE_1).unwrap();
        assert_eq!(result, 6);
    }
}
