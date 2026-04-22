use std::collections::HashSet;

use crate::data::PartJoin;

#[derive(Clone, Debug, Default)]
pub struct PartJoiner {
    joins: HashSet<PartJoin>,
}

impl PartJoiner {
    pub fn register_part_join(&mut self, part1: usize, part2: usize, with_node: usize) {
        let (min, max) = (part1.min(part2), part1.max(part2));
        let pj = PartJoin {
            with_node,
            from: max,
            to: min,
            ordering: self.joins.len() as u32,
        };
        self.joins.insert(pj);
    }

    pub fn take(self) -> HashSet<PartJoin> {
        self.joins
    }
}
