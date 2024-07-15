use slotmap::{new_key_type, SecondaryMap, SlotMap};

new_key_type! {
    pub struct CircutNode;
}

pub struct CircutItem;

pub struct Circut {
    nodes: SlotMap<CircutNode, CircutItem>,
    /// Inputs of a node, with the source node and it's ouput index, if connected.
    inputs: SecondaryMap<CircutNode, Vec<Option<(CircutNode, usize)>>>,
    /// Outputs of a node, with the destination node and it's input index, if connected.
    outputs: SecondaryMap<CircutNode, Vec<Option<(CircutNode, usize)>>>,
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
        let outputs = &mut self.outputs[src.0];
        outputs.extend((outputs.len()..=src.1).map(|_| None));
        if let Some(_connection) = outputs[src.1].replace(dst) {
            println!("Output was already connected");
        }

        let inputs = &mut self.inputs[dst.0];
        inputs.extend((inputs.len()..=dst.1).map(|_| None));
        if let Some(_connection) = inputs[dst.1].replace(src) {
            println!("Input was already connected");
        }
    }
}
