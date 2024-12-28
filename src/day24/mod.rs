use std::{collections::VecDeque, rc::Rc};

use aoc_utils::{cache::Cache, hashbrown::HashMap, AocError};
use itertools::Itertools;

mod parser;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(input: &str) -> anyhow::Result<u64> {
    let (x, y, device) = self::parser::parse(input)?;
    let result = evaluate_device(&device, x, y);
    Ok(result)
}

fn evaluate_device(device: &Device, x: u64, y: u64) -> u64 {
    fn recurse(device: &Device, index: u32, cache: &mut impl Cache<usize, bool>) -> bool {
        if let Some(&value) = cache.get(&(index as usize)) {
            return value;
        }

        let Some(node) = device.get(index) else {
            return false;
        };

        let value = match node.gate {
            Some(Gate::And(a, b)) => recurse(device, a, cache) && recurse(device, b, cache),
            Some(Gate::Or(a, b)) => recurse(device, a, cache) || recurse(device, b, cache),
            Some(Gate::Xor(a, b)) => recurse(device, a, cache) ^ recurse(device, b, cache),
            _ => false,
        };

        cache.insert(index as usize, value);
        value
    }

    let mut values = vec![None; device.graph.len()];

    for (bit, &index) in device.x.iter().enumerate() {
        values[index as usize] = Some(x >> bit & 1 != 0);
    }

    for (bit, &index) in device.y.iter().enumerate() {
        values[index as usize] = Some(y >> bit & 1 != 0);
    }

    let mut z = 0;
    for (bit, &index) in device.z.iter().enumerate() {
        let value = recurse(device, index, &mut values);
        z |= (value as u64) << bit as u32;
    }

    z
}

fn part_2(input: &str) -> anyhow::Result<String> {
    let (_, _, device) = self::parser::parse(input)?;
    let swapped_wires = find_swapped_wires(&device);

    let result = swapped_wires
        .into_iter()
        .map(|index| format!("{}", device[index].wire))
        .sorted_unstable()
        .join(",");
    Ok(result)
}

fn find_swapped_wires(device: &Device) -> Vec<u32> {
    let mut swapped = Vec::new();
    let mut queue = VecDeque::from_iter(device.z.iter().copied());

    let mut emit = |index: u32, _reason: &'static str| {
        if !swapped.contains(&index) {
            swapped.push(index);
            // tracing::trace!(wire = %device[index].wire.clone(), reason, "swapped wire");
        }
    };

    while let Some(index) = queue.pop_front() {
        let node = &device[index];

        if node.is_output() {
            check_output_node(device, node, &mut emit);
            check_node(device, node, &mut emit);
        } else {
            check_node(device, node, &mut emit);
        }

        queue.extend(node.inputs().iter().flat_map(|inputs| inputs));
    }

    swapped
}

fn check_output_node(device: &Device, node: &Node, emit: &mut impl FnMut(u32, &'static str)) {
    // Output nodes must be output wires
    let Wire::Z(bit) = node.wire else {
        emit(node.index, "not an output node");
        return;
    };

    let is_last_output_bit = bit as usize == device.z.len() - 1;
    match node.gate {
        Some(Gate::Or(..)) if is_last_output_bit => {
            // Last output bit must be a carry
        }
        Some(Gate::Xor(..)) if !is_last_output_bit => {
            // All other output bits must be a sum
        }
        _ => {
            emit(node.index, "invalid gate kind for output node");
            return;
        }
    }

    // Output nodes must have input
    let Some(inputs) = node.inputs() else {
        emit(node.index, "missing inputs");
        return;
    };
    let inputs = inputs.map(|index| device[index].wire.clone());

    if bit == 0 {
        // `z00` must be `x00 XOR y00`
        if !matches!(
            inputs,
            [Wire::X(..), Wire::Y(..)] | [Wire::Y(..), Wire::X(..)]
        ) {
            emit(node.index, "invalid z00 half adder");
        }
    } else {
        // Other output nodes cannot take input directly
        if !matches!(inputs, [Wire::Name(..), Wire::Name(..)]) {
            emit(node.index, "output node taking input directly");
        }
    }
}

fn check_node(device: &Device, node: &Node, emit: &mut impl FnMut(u32, &'static str)) {
    fn has_x00_input(device: &Device, index: u32) -> bool {
        let node = &device[index];
        for &index in node.inputs().iter().flat_map(|inputs| inputs) {
            let input = &device[index];
            if input.wire == Wire::X(0) {
                return true;
            }
        }

        false
    }

    match node.gate {
        Some(Gate::Or(a, b)) => {
            // `OR` gates must only have `AND` inputs
            if !matches!(device[a].gate, Some(Gate::And(..))) {
                emit(a, "carry node taking non-AND first input");
                return;
            }

            if !matches!(device[b].gate, Some(Gate::And(..))) {
                emit(b, "carry node taking non-AND second input");
                return;
            }
        }
        Some(Gate::And(a, b)) => {
            // Consecutive `AND` gates can only happen after `x00`
            if matches!(device[a].gate, Some(Gate::And(..))) && !has_x00_input(device, a) {
                emit(a, "consecutive AND on first input");
                return;
            }

            if matches!(device[b].gate, Some(Gate::And(..))) && !has_x00_input(device, b) {
                emit(b, "consecutive AND on second input");
                return;
            }
        }
        Some(Gate::Xor(a, b)) => {
            // `XOR` gates with `OR` and `XOR` inputs must always be outputs
            if matches!(
                [device[a].gate, device[b].gate],
                [Some(Gate::Or(..)), Some(Gate::Xor(..))]
                    | [Some(Gate::Xor(..)), Some(Gate::Or(..))]
            ) && !matches!(node.wire, Wire::Z(..))
            {
                emit(node.index, "non-output carry-adder node");
                return;
            }
        }
        _ => {}
    }
}

#[derive(Debug, Clone)]
struct Device {
    index: HashMap<Wire, u32>,
    graph: Vec<Node>,
    x: Vec<u32>,
    y: Vec<u32>,
    z: Vec<u32>,
}

impl Device {
    const SIZE: usize = 64;

    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            graph: Vec::new(),
            x: Vec::with_capacity(Self::SIZE),
            y: Vec::with_capacity(Self::SIZE),
            z: Vec::with_capacity(Self::SIZE),
        }
    }

    pub fn insert(
        &mut self,
        wire: Wire,
        gate_kind: GateKind,
        inputs: [Wire; 2],
    ) -> Result<(), AocError> {
        let inputs = inputs.map(|wire| self.get_or_insert(wire, None));
        let index = self.get_or_insert(wire.clone(), None);

        let node = &mut self.graph[index as usize];

        if node.gate.is_some() {
            return Err(AocError::message(format!(
                "Wire '{wire}' is defined multiple times"
            )));
        }

        node.gate = Some(Gate::new(gate_kind, inputs));

        Ok(())
    }

    pub fn get(&self, index: u32) -> Option<&Node> {
        self.graph.get(index as usize)
    }

    fn get_or_insert(&mut self, wire: Wire, gate: Option<Gate>) -> u32 {
        if let Some(index) = self.index.get(&wire) {
            *index
        } else {
            let index = self.graph.len() as u32;
            let gate = match wire {
                Wire::X(bit) => {
                    self.set_x(bit, index);
                    Some(Gate::Input)
                }
                Wire::Y(bit) => {
                    self.set_y(bit, index);
                    Some(Gate::Input)
                }
                Wire::Z(bit) => {
                    self.set_z(bit, index);
                    gate
                }
                _ => gate,
            };

            let node = Node::new(index, wire.clone(), gate);

            self.graph.push(node);
            self.index.insert(wire, index);

            index
        }
    }

    fn set_x(&mut self, bit: u32, index: u32) {
        Self::ensure_valid_bit(&mut self.x, bit);
        self.x[bit as usize] = index;
    }

    fn set_y(&mut self, bit: u32, index: u32) {
        Self::ensure_valid_bit(&mut self.y, bit);
        self.y[bit as usize] = index;
    }

    fn set_z(&mut self, bit: u32, index: u32) {
        Self::ensure_valid_bit(&mut self.z, bit);
        self.z[bit as usize] = index;
    }

    fn ensure_valid_bit(vec: &mut Vec<u32>, bit: u32) {
        let bit = bit as usize;

        if bit >= vec.len() {
            let new_len = bit + 1;
            vec.resize(new_len, 0);
        }
    }
}

impl std::ops::Index<u32> for Device {
    type Output = Node;

    fn index(&self, index: u32) -> &Self::Output {
        &self.graph[index as usize]
    }
}

impl std::ops::IndexMut<u32> for Device {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.graph[index as usize]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Wire {
    X(u32),
    Y(u32),
    Z(u32),
    Name(Rc<str>),
}

impl Wire {
    pub fn new(name: &str) -> Self {
        Self::try_from_bit_name(name).unwrap_or_else(|| Self::Name(name.into()))
    }

    fn try_from_bit_name(name: &str) -> Option<Self> {
        let mut chars = name.chars();
        let first = chars.next()?;
        let bit = u32::from_str_radix(chars.as_str(), 10).ok()?;

        match first {
            'x' => Some(Self::X(bit)),
            'y' => Some(Self::Y(bit)),
            'z' => Some(Self::Z(bit)),
            _ => None,
        }
    }
}

impl std::fmt::Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X(bit) => write!(f, "x{bit:02}"),
            Self::Y(bit) => write!(f, "y{bit:02}"),
            Self::Z(bit) => write!(f, "z{bit:02}"),
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    index: u32,
    wire: Wire,
    gate: Option<Gate>,
}

impl Node {
    pub fn new(index: u32, wire: Wire, gate: Option<Gate>) -> Self {
        Self { index, wire, gate }
    }

    pub const fn is_output(&self) -> bool {
        matches!(self.wire, Wire::Z(..))
    }

    pub fn inputs(&self) -> Option<[u32; 2]> {
        self.gate.as_ref().and_then(Gate::inputs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Gate {
    Input,
    And(u32, u32),
    Or(u32, u32),
    Xor(u32, u32),
}

impl Gate {
    pub fn new(kind: GateKind, inputs: [u32; 2]) -> Self {
        match kind {
            GateKind::And => Self::And(inputs[0], inputs[1]),
            GateKind::Or => Self::Or(inputs[0], inputs[1]),
            GateKind::Xor => Self::Xor(inputs[0], inputs[1]),
        }
    }

    pub const fn kind(&self) -> Option<GateKind> {
        match self {
            Self::And(..) => Some(GateKind::And),
            Self::Or(..) => Some(GateKind::Or),
            Self::Xor(..) => Some(GateKind::Xor),
            _ => None,
        }
    }

    pub fn inputs(&self) -> Option<[u32; 2]> {
        match self {
            Self::And(a, b) => Some([*a, *b]),
            Self::Or(a, b) => Some([*a, *b]),
            Self::Xor(a, b) => Some([*a, *b]),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateKind {
    And,
    Or,
    Xor,
}

#[allow(dead_code)]
fn print_graph() -> anyhow::Result<()> {
    use std::{fs::File, io::Write};

    let (_, _, device) = self::parser::parse(INPUT)?;
    let mut file = File::create("output/day24.dot")?;

    writeln!(file, "digraph {{")?;

    for (index, node) in device.graph.iter().enumerate() {
        match node.gate.as_ref().and_then(Gate::kind) {
            Some(kind) => writeln!(file, "  {index} [ label = \"{} ({:?})\" ]", node.wire, kind)?,
            None => writeln!(file, "  {index} [ label = \"{}\" ]", node.wire)?,
        }
    }

    for (index, node) in device.graph.iter().enumerate() {
        match node.gate {
            Some(Gate::And(a, b)) => {
                writeln!(file, "  {a} -> {index}")?;
                writeln!(file, "  {b} -> {index}")?;
            }
            Some(Gate::Or(a, b)) => {
                writeln!(file, "  {a} -> {index}")?;
                writeln!(file, "  {b} -> {index}")?;
            }
            Some(Gate::Xor(a, b)) => {
                writeln!(file, "  {a} -> {index}")?;
                writeln!(file, "  {b} -> {index}")?;
            }
            _ => {}
        }
    }

    writeln!(file, "}}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");
    const EXAMPLE_2: &str = include_str!("example.2.txt");

    #[rstest]
    #[case(EXAMPLE_1, 4)]
    #[case(EXAMPLE_2, 2024)]
    fn test_part_1(#[case] input: &str, #[case] expected: u64) {
        let result = part_1(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let _result = part_2(EXAMPLE_1).unwrap();
    }
}
