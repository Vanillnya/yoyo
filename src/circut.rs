use slotmap::{new_key_type, SecondaryMap, SlotMap};

new_key_type! {
    pub struct CircutNode;
}

pub struct CircutItem;

pub struct Circut {
    nodes: SlotMap<CircutNode, CircutItem>,
    /// Inputs of a node, with the source node and its ouput index, if connected.
    inputs: SecondaryMap<CircutNode, Vec<Option<(CircutNode, usize)>>>,
    /// Outputs of a node, with the destination nodes and their input indices.
    outputs: SecondaryMap<CircutNode, Vec<Vec<(CircutNode, usize)>>>,
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
}
