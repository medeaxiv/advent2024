use std::collections::BinaryHeap;

use aoc_utils::{
    hashbrown::{hash_map::Entry, HashMap},
    petgraph::{
        graph::{EdgeIndex, NodeIndex},
        visit::EdgeRef,
    },
};

use super::Map;

pub fn search(map: &Map) -> ResultSet {
    let (mut visited_set, mut queue) = begin_search(map);

    while let Some(next) = queue.pop() {
        if !visited_set.visit_node(next.source, next.destination, next.edge, next.cost) {
            continue;
        }

        let node = next.destination;
        let cost = next.cost;
        for neighbor in map.graph.neighbors(node) {
            for edge in map.graph.edges_connecting(node, neighbor) {
                let cost = cost + edge.weight().cost();
                let edge = Edge {
                    source: node,
                    destination: neighbor,
                    edge: edge.id(),
                    cost,
                };

                queue.push(edge);
            }
        }
    }

    ResultSet {
        store: visited_set.store,
    }
}

fn begin_search(map: &Map) -> (VisitedSet, BinaryHeap<Edge>) {
    let mut visited_set = VisitedSet::new();
    visited_set.visit_root(map.start);

    let mut queue = BinaryHeap::new();

    for neighbor in map.graph.neighbors(map.start) {
        for edge in map.graph.edges_connecting(map.start, neighbor) {
            let edge = Edge {
                source: map.start,
                destination: neighbor,
                edge: edge.id(),
                cost: edge.weight().cost(),
            };

            queue.push(edge);
        }
    }

    (visited_set, queue)
}

struct Edge {
    source: NodeIndex,
    destination: NodeIndex,
    edge: EdgeIndex,
    cost: u64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        // Intentionally a reverse comparison
        other.cost.eq(&self.cost)
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Intentionally a reverse comparison
        other.cost.cmp(&self.cost)
    }
}

pub struct ResultSet {
    store: HashMap<NodeIndex, VisitedSetEntry>,
}

impl ResultSet {
    pub fn cost(&self, node: NodeIndex) -> Option<u64> {
        self.store.get(&node).map(|entry| entry.cost())
    }

    pub fn edges_to(&self, node: NodeIndex) -> std::slice::Iter<'_, (NodeIndex, EdgeIndex)> {
        if let Some(entry) = self.store.get(&node) {
            entry.edges()
        } else {
            (&[]).iter()
        }
    }
}

struct VisitedSet {
    store: HashMap<NodeIndex, VisitedSetEntry>,
}

impl VisitedSet {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn visit_root(&mut self, node: NodeIndex) {
        self.store.insert(node, VisitedSetEntry::Root);
    }

    pub fn visit_node(
        &mut self,
        source: NodeIndex,
        destination: NodeIndex,
        edge: EdgeIndex,
        cost: u64,
    ) -> bool {
        match self.store.entry(destination) {
            Entry::Vacant(vacant) => {
                let entry = VisitedSetEntryNode::new(source, edge, cost);
                vacant.insert(VisitedSetEntry::Node(entry));
                true
            }
            Entry::Occupied(mut occupied) => occupied.get_mut().add_edge(source, edge, cost),
        }
    }
}

enum VisitedSetEntry {
    Root,
    Node(VisitedSetEntryNode),
}

impl VisitedSetEntry {
    pub const fn cost(&self) -> u64 {
        match self {
            Self::Root => 0,
            Self::Node(node) => node.cost,
        }
    }

    pub fn edges(&self) -> std::slice::Iter<'_, (NodeIndex, EdgeIndex)> {
        match self {
            Self::Root => (&[]).iter(),
            Self::Node(node) => node.edges(),
        }
    }

    pub fn add_edge(&mut self, node: NodeIndex, edge: EdgeIndex, cost: u64) -> bool {
        match self {
            Self::Root => false,
            Self::Node(entry) => entry.add_edge(node, edge, cost),
        }
    }
}

struct VisitedSetEntryNode {
    cost: u64,
    edges: Vec<(NodeIndex, EdgeIndex)>,
}

impl VisitedSetEntryNode {
    pub fn new(node: NodeIndex, edge: EdgeIndex, cost: u64) -> Self {
        Self {
            cost,
            edges: vec![(node, edge)],
        }
    }

    pub fn add_edge(&mut self, node: NodeIndex, edge: EdgeIndex, cost: u64) -> bool {
        if cost < self.cost {
            self.cost = cost;
            self.edges.clear();
            self.edges.push((node, edge));
            true
        } else if cost == self.cost {
            self.edges.push((node, edge));
            false
        } else {
            false
        }
    }

    pub fn edges(&self) -> std::slice::Iter<'_, (NodeIndex, EdgeIndex)> {
        self.edges.iter()
    }
}
