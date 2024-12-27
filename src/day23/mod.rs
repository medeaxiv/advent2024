use aoc_utils::{
    hashbrown::{HashMap, HashSet},
    numerics::min_max,
    petgraph::{graph::NodeIndex, Graph, Undirected},
    AocError,
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

fn part_1(input: &str) -> anyhow::Result<i64> {
    let graph = parse(input)?;

    let mut count = 0;
    for (a, b, c) in triplets(&graph) {
        let a_is_candidate = graph
            .node_weight(a)
            .is_some_and(|name| name.starts_with('t'));
        let b_is_candidate = graph
            .node_weight(b)
            .is_some_and(|name| name.starts_with('t'));
        let c_is_candidate = graph
            .node_weight(c)
            .is_some_and(|name| name.starts_with('t'));

        if a_is_candidate || b_is_candidate || c_is_candidate {
            count += 1;
        }
    }

    Ok(count)
}

fn triplets<'g>(
    graph: &'g Graph<&str, (), Undirected>,
) -> impl Iterator<Item = (NodeIndex, NodeIndex, NodeIndex)> + use<'g> {
    graph
        .node_indices()
        .flat_map(|node| {
            graph
                .neighbors(node)
                .tuple_combinations()
                .filter(|&(b, c)| graph.contains_edge(b, c))
                .map(move |(b, c)| make_triplet(node, b, c))
        })
        .unique()
}

fn make_triplet(a: NodeIndex, b: NodeIndex, c: NodeIndex) -> (NodeIndex, NodeIndex, NodeIndex) {
    let (min_ab, max_ab) = min_max(a, b);
    let (min, mid) = min_max(min_ab, c);
    let (mid, max) = min_max(max_ab, mid);

    debug_assert!(min < mid);
    debug_assert!(mid < max);
    debug_assert!(min < max);

    (min, mid, max)
}

fn part_2(input: &str) -> anyhow::Result<String> {
    let graph = parse(input)?;
    let clique = bron_kerbosch(&graph);

    let mut result = String::new();
    for (index, name) in clique
        .iter()
        .map(|&node| graph.node_weight(node).expect("Node must be in graph"))
        .sorted()
        .enumerate()
    {
        if index != 0 {
            result.push(',');
        }

        result.push_str(name);
    }

    Ok(result)
}

fn bron_kerbosch(graph: &Graph<&str, (), Undirected>) -> HashSet<NodeIndex> {
    fn neighbor_set(graph: &Graph<&str, (), Undirected>, node: NodeIndex) -> HashSet<NodeIndex> {
        graph.neighbors(node).collect()
    }

    fn recurse(
        graph: &Graph<&str, (), Undirected>,
        level: usize,
        largest: &mut HashSet<NodeIndex>,
        r: HashSet<NodeIndex>,
        mut p: HashSet<NodeIndex>,
        mut x: HashSet<NodeIndex>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > largest.len() {
                *largest = r;
            }

            return;
        }

        while let Some(&v) = p.iter().next() {
            let next_r = {
                let mut r = r.clone();
                r.insert(v);
                r
            };

            let neighbors = neighbor_set(graph, v);
            let next_p = p.intersection(&neighbors).copied().collect();
            let next_x = x.intersection(&neighbors).copied().collect();
            recurse(graph, level + 1, largest, next_r, next_p, next_x);

            p.remove(&v);
            x.insert(v);
        }
    }

    let p = graph.node_indices().collect();
    let r = HashSet::new();
    let x = HashSet::new();
    let mut largest = HashSet::new();
    recurse(graph, 0, &mut largest, r, p, x);

    largest
}

fn parse(input: &str) -> anyhow::Result<Graph<&str, (), Undirected>> {
    let mut nodes = HashMap::new();
    let mut graph = Graph::new_undirected();
    for line in input.lines() {
        let Some((a, b)) = line.split_once('-') else {
            return Err(AocError::InvalidInput.into());
        };

        let a = *nodes.entry(a).or_insert_with(|| graph.add_node(a));
        let b = *nodes.entry(b).or_insert_with(|| graph.add_node(b));
        graph.add_edge(a, b, ());
    }

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[rstest]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, 7)
    }

    #[rstest]
    fn test_part_2() {
        aoc_utils::tracing::setup_tracing(crate::AOC_LOG);
        let result = part_2(EXAMPLE_1).unwrap();
        assert_eq!(result, "co,de,ka,ta");
    }

    // #[rstest]
    // #[case(INPUT, "")]
    // #[case(EXAMPLE_1, "example.1")]
    // fn print_graph(#[case] input: &str, #[case] name: &str) -> std::io::Result<()> {
    //     use aoc_utils::petgraph::dot::{Config, Dot};
    //     use std::fs::File;
    //     use std::io::Write;

    //     let graph = parse(input).unwrap();

    //     let path = match name {
    //         "" => "output/day23.dot".to_string(),
    //         _ => format!("output/day23-{name}.dot"),
    //     };

    //     let mut file = File::create(&path).unwrap();
    //     write!(
    //         file,
    //         "{:?}",
    //         Dot::with_config(&graph, &[Config::EdgeNoLabel])
    //     )
    // }
}
