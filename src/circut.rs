use std::{collections::VecDeque, marker::PhantomData};

use egui::ahash::{HashSet, HashSetExt};
use slotmap::{new_key_type, SecondaryMap, SlotMap};

new_key_type! {
    pub struct CircutNode;
}

pub struct CircutItem;

pub struct Circut {
    nodes: SlotMap<CircutNode, CircutItem>,
    /// Inputs of a node, with the source node and its ouput index, if connected.
    pub(crate) inputs: SecondaryMap<CircutNode, Vec<Option<(CircutNode, usize)>>>,
    /// Outputs of a node, with the destination nodes and their input indices.
    pub(crate) outputs: SecondaryMap<CircutNode, Vec<Vec<(CircutNode, usize)>>>,
}

impl Circut {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            inputs: SecondaryMap::new(),
            outputs: SecondaryMap::new(),
        }
    }

    pub fn add_item(&mut self, item: CircutItem) -> CircutNode {
        let node = self.nodes.insert(item);
        self.inputs.insert(node, Vec::new());
        self.outputs.insert(node, Vec::new());
        node
    }

    pub fn connect(&mut self, src: (CircutNode, usize), dst: (CircutNode, usize)) {
        let inputs = &mut self.inputs[dst.0];
        inputs.extend((inputs.len()..=dst.1).map(|_| None));
        if let Some(_connection) = inputs[dst.1].replace(src) {
            println!("Input was already connected");
            return;
        }

        let outputs = &mut self.outputs[src.0];
        outputs.extend((outputs.len()..=src.1).map(|_| Vec::new()));
        outputs[src.1].push(dst);
    }

    pub fn sinks<'a>(&'a self) -> impl Iterator<Item = CircutNode> + 'a {
        self.outputs
            .iter()
            .filter(|node| node.1.iter().all(|port| port.is_empty()))
            .map(|(node, _)| node)
    }
}

mod dir {
    /// Go from a node down further to its input sources
    pub struct Inputs;
}

pub struct Bfs<'c, Data, Walk: Iterator<Item = (CircutNode, Data)>> {
    circut: &'c Circut,
    queue: VecDeque<(CircutNode, Data)>,
    visited: HashSet<CircutNode>,
    visitor: fn(&'c Circut, CircutNode, &Data) -> Walk,
}

impl<'c, Data, Walk: Iterator<Item = (CircutNode, Data)>> Bfs<'c, Data, Walk> {
    pub fn with_starters<I>(
        circut: &'c Circut,
        starters: I,
        visitor: fn(&'c Circut, CircutNode, &Data) -> Walk,
    ) -> Self
    where
        I: Iterator<Item = (CircutNode, Data)>,
    {
        Self {
            circut,
            queue: VecDeque::from_iter(starters),
            visited: HashSet::new(),
            visitor,
        }
    }
}

impl<'c, Data, Walk: Iterator<Item = (CircutNode, Data)>> Iterator for Bfs<'c, Data, Walk> {
    type Item = (CircutNode, Data);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, data)) = self.queue.pop_front() {
            for (input, sub_data) in (self.visitor)(self.circut, node, &data) {
                if !self.visited.contains(&input) {
                    self.visited.insert(input);
                    self.queue.push_back((input, sub_data));
                }
            }
            Some((node, data))
        } else {
            None
        }
    }
}
