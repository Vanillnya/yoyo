use slotmap::SecondaryMap;

use crate::circut::{Circut, CircutNode};

pub struct CircutLayout {
    positions: SecondaryMap<CircutNode, (f32, f32)>,
}

impl CircutLayout {
    pub fn layout(circut: &Circut) -> Self {
        Self {
            positions: SecondaryMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::circut::{Circut, CircutItem};

    use super::CircutLayout;

    #[test]
    fn sr_latch() {
        let mut circut = Circut::new();
        let rst = circut.add_item(CircutItem);
        let set = circut.add_item(CircutItem);
        let nor1 = circut.add_item(CircutItem);
        let nor2 = circut.add_item(CircutItem);
        let q = circut.add_item(CircutItem);
        let not_q = circut.add_item(CircutItem);
        circut.connect((rst, 0), (nor1, 0));
        circut.connect((set, 0), (nor2, 1));
        circut.connect((nor1, 0), (nor2, 0));
        circut.connect((nor2, 0), (nor1, 1));
        circut.connect((nor1, 0), (q, 0));
        circut.connect((nor2, 0), (not_q, 0));

        let layout = CircutLayout::layout(&circut);
    }
}
