use slotmap::SecondaryMap;

use crate::circut::{Bfs, Circut, CircutNode};

pub struct CircutLayout {
    pub positions: SecondaryMap<CircutNode, (f32, f32)>,
}

impl CircutLayout {
    pub fn layout(circut: &Circut) -> Self {
        // Distance
        let mut end_distance = SecondaryMap::new();

        let bfs = Bfs::with_starters(
            circut,
            circut.sinks().map(|node| (node, 0 as usize)),
            |c, n, acc| {
                let data = acc + 1;
                c.inputs[n]
                    .iter()
                    .flatten()
                    .map(move |&(node, _)| (node, data))
            },
        );

        for (node, distance) in bfs {
            end_distance.insert(node, distance);
        }

        // Stacking
        let mut positions = SecondaryMap::new();
        let mut stack_height = Vec::new();

        for (node, distance) in end_distance {
            stack_height.extend((stack_height.len()..=distance).map(|_| 0));
            let height = stack_height[distance];
            stack_height[distance] += 1;
            positions.insert(node, (distance as f32 * -1.5, height as f32 * 1.));
        }

        Self { positions }
    }
}

pub mod constructs {
    use crate::{
        circut::{Circut, CircutItem},
        symbol::SymbolKind,
    };

    pub fn sr_latch() -> Circut {
        let mut circut = Circut::new();

        let rst = circut.add_item(CircutItem {
            symbol: SymbolKind::Input,
        });
        let set = circut.add_item(CircutItem {
            symbol: SymbolKind::Input,
        });
        let nor1 = circut.add_item(CircutItem {
            symbol: SymbolKind::Nor,
        });
        let nor2 = circut.add_item(CircutItem {
            symbol: SymbolKind::Nor,
        });
        let q = circut.add_item(CircutItem {
            symbol: SymbolKind::Output,
        });
        let not_q = circut.add_item(CircutItem {
            symbol: SymbolKind::Output,
        });

        circut.connect((rst, 0), (nor1, 0));
        circut.connect((set, 0), (nor2, 1));
        circut.connect((nor1, 0), (nor2, 0));
        circut.connect((nor2, 0), (nor1, 1));
        circut.connect((nor1, 0), (q, 0));
        circut.connect((nor2, 0), (not_q, 0));

        circut
    }
}

#[cfg(test)]
mod tests {
    use super::{constructs, CircutLayout};

    #[test]
    fn sr_latch() {
        let circut = constructs::sr_latch();

        let layout = CircutLayout::layout(&circut);
    }
}
