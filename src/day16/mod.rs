use allocator_api2::vec::Vec;
use aoc_utils::{
    direction::{Direction, Orientation},
    grid::{Coordinates, Grid},
    hashbrown::{HashMap, HashSet},
    petgraph::{graph::NodeIndex, Graph, Undirected},
    AocError,
};

mod search;

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
    let cost = find_cheapest_path_cost(&map).ok_or(AocError::message("Unable to find a path"))?;
    Ok(cost)
}

fn find_cheapest_path_cost(map: &Map) -> Option<u64> {
    let results = self::search::search(map);

    let horizontal_end_cost = results.cost(map.end.0);
    let vertical_end_cost = results.cost(map.end.1);
    horizontal_end_cost.and_then(|h| vertical_end_cost.map(|v| std::cmp::min(h, v)))
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let map = parse(input)?;
    let count =
        find_cheapest_path_cell_count(&map).ok_or(AocError::message("Unable to find a path"))?;
    Ok(count)
}

fn find_cheapest_path_cell_count(map: &Map) -> Option<u64> {
    let results = self::search::search(map);

    let horizontal_end_cost = results.cost(map.end.0);
    let vertical_end_cost = results.cost(map.end.1);
    let node = horizontal_end_cost
        .and_then(|h| vertical_end_cost.map(|v| if h < v { map.end.0 } else { map.end.1 }))?;

    let mut cells = HashSet::new();
    let mut stack = Vec::new();
    stack.extend(results.edges_to(node));
    while let Some((node, edge)) = stack.pop() {
        stack.extend(results.edges_to(node));

        if let Some((a, b)) = map.graph.edge_endpoints(edge) {
            let a = map.graph.node_weight(a).unwrap().coordinates();
            let b = map.graph.node_weight(b).unwrap().coordinates();

            if a.x == b.x {
                for y in a.y..=b.y {
                    cells.insert(Coordinates::new(a.x, y));
                }
            } else if a.y == b.y {
                for x in a.x..=b.x {
                    cells.insert(Coordinates::new(x, a.y));
                }
            }
        }
    }

    Some(cells.len() as u64)
}

fn parse(input: &str) -> anyhow::Result<Map> {
    let line_length = input
        .lines()
        .next()
        .ok_or(AocError::InvalidInput)?
        .chars()
        .count();

    let mut lines = 0;
    let mut start = Coordinates::zeros();
    let mut end = Coordinates::zeros();
    let mut data = Vec::with_capacity(line_length * line_length);
    for (y, line) in input.lines().enumerate() {
        lines += 1;

        for (x, c) in line.chars().enumerate() {
            let coordinates = Coordinates::new(x as u32, y as u32);

            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'S' => {
                    start = coordinates;
                    Tile::Empty
                }
                'E' => {
                    end = coordinates;
                    Tile::Empty
                }
                _ => return Err(AocError::InvalidInput.into()),
            };

            data.push(tile);
        }
    }

    let grid = Grid::from_vec(line_length as u32, lines, data);
    let map = Map::new(grid, start, end);

    Ok(map)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

struct Map {
    graph: Graph<Node, Edge, Undirected>,
    start: NodeIndex,
    end: (NodeIndex, NodeIndex),
}

impl Map {
    pub fn new(grid: Grid<Tile>, start: Coordinates, end: Coordinates) -> Self {
        let mut graph = Graph::new_undirected();
        let mut start_node = NodeIndex::end();
        let mut end_nodes = (NodeIndex::end(), NodeIndex::end());

        let mut nodes = HashMap::new();
        for y in 0..grid.height() {
            let mut previous: Option<(NodeIndex, Coordinates)> = None;
            for x in 0..grid.width() {
                let coordinates = Coordinates::new(x, y);
                if matches!(grid.get(coordinates), Some(Tile::Wall)) {
                    previous = None;
                    continue;
                }

                let neighbors = [
                    Direction::Left.checked_apply_movement(coordinates, 1),
                    Direction::Right.checked_apply_movement(coordinates, 1),
                    Direction::Up.checked_apply_movement(coordinates, 1),
                    Direction::Down.checked_apply_movement(coordinates, 1),
                ];

                let neighbors = neighbors.map(|neighbor| {
                    neighbor.and_then(|neighbor| {
                        grid.get(neighbor)
                            .filter(|tile| matches!(tile, Tile::Empty))
                            .copied()
                    })
                });

                let has_horizontal_neighbors = neighbors[0].is_some() || neighbors[1].is_some();
                let has_vertical_neighbors = neighbors[2].is_some() || neighbors[3].is_some();
                let is_dead_end = neighbors.iter().filter(|n| n.is_some()).count() == 1;

                if (has_horizontal_neighbors && has_vertical_neighbors) || is_dead_end {
                    let horizontal_node = graph.add_node(Node::Horizontal(coordinates));
                    let vertical_node = graph.add_node(Node::Vertical(coordinates));
                    graph.add_edge(horizontal_node, vertical_node, Edge::Turn);
                    nodes.insert(coordinates, (horizontal_node, vertical_node));

                    if coordinates == start {
                        start_node = horizontal_node;
                    }

                    if coordinates == end {
                        end_nodes = (horizontal_node, vertical_node);
                    }

                    if let Some((last_node, last_coordinates)) = previous {
                        let distance = coordinates.x.abs_diff(last_coordinates.x);
                        graph.add_edge(last_node, horizontal_node, Edge::Move(distance));
                    }

                    previous = Some((horizontal_node, coordinates));
                }
            }
        }

        for (&coordinates, &(_, last_node)) in nodes.iter() {
            let mut next = Direction::Down.apply_movement(coordinates, 1);

            loop {
                if matches!(grid.get(next), Some(Tile::Wall) | None) {
                    break;
                }

                if let Some(&(_, node)) = nodes.get(&next) {
                    let distance = coordinates.y.abs_diff(next.y);
                    graph.add_edge(last_node, node, Edge::Move(distance));
                    break;
                }

                next = Direction::Down.apply_movement(next, 1);
            }
        }

        Self {
            graph,
            start: start_node,
            end: end_nodes,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Node {
    Horizontal(Coordinates),
    Vertical(Coordinates),
}

impl Node {
    pub const fn coordinates(&self) -> Coordinates {
        match self {
            Self::Horizontal(coordinates) => *coordinates,
            Self::Vertical(coordinates) => *coordinates,
        }
    }

    pub const fn orientation(&self) -> Orientation {
        match self {
            Self::Horizontal(..) => Orientation::Horizontal,
            Self::Vertical(..) => Orientation::Vertical,
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} ({})",
            self.coordinates(),
            self.orientation().display_char(),
        )
    }
}

#[derive(Clone, Copy)]
enum Edge {
    Turn,
    Move(u32),
}

impl Edge {
    pub const fn cost(&self) -> u64 {
        match self {
            Self::Turn => 1000,
            Self::Move(distance) => *distance as u64,
        }
    }
}

impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Turn => write!(f, "Turn"),
            Self::Move(distance) => write!(f, "{distance}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");
    const EXAMPLE_2: &str = include_str!("example.2.txt");

    #[rstest]
    #[case(EXAMPLE_1, 7036)]
    #[case(EXAMPLE_2, 11048)]
    fn test_part_1(#[case] input: &str, #[case] expected: u64) {
        let result = part_1(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(EXAMPLE_1, 45)]
    #[case(EXAMPLE_2, 64)]
    fn test_part_2(#[case] input: &str, #[case] expected: u64) {
        let result = part_2(input).unwrap();
        assert_eq!(result, expected);
    }

    // #[rstest]
    // #[case(INPUT, "")]
    // #[case(EXAMPLE_1, "example.1")]
    // #[case(EXAMPLE_2, "example.2")]
    // fn print_graph(#[case] input: &str, #[case] name: &str) -> std::io::Result<()> {
    //     use aoc_utils::petgraph::dot::Dot;
    //     use std::fs::File;
    //     use std::io::Write;

    //     let map = parse(input).unwrap();

    //     let path = match name {
    //         "" => "output/day16.dot".to_string(),
    //         _ => format!("output/day16-{name}.dot"),
    //     };

    //     let mut file = File::create(&path).unwrap();
    //     write!(file, "{:?}", Dot::new(&map.graph))
    // }
}
