use super::hook_result::{Edges, HookResult, Peculiarity};
use crate::{
    common::V,
    flow::{actions::Action, Flow},
};
use Action::*;
use HookError::*;

use std::collections::HashMap;

#[derive(Debug)]
pub enum HookError {
    Empty,
    BadStarter,
    StarterInTheMiddle,
    ChainStart,
    TriedToWorkAfterFastenOff,
}

impl From<HookError> for String {
    fn from(value: HookError) -> Self {
        format!("{value:?}")
    }
}

/// Responsible for building the graph used in the simulation
#[derive(Clone)]
pub struct Hook {
    edges: Edges,
    peculiar: HashMap<usize, Peculiarity>,
    round_count: usize,
    round_left: usize,
    anchor: usize,
    cursor: usize,
    fastened_off: bool,
    /// Constains first and last stitch of each round. Treated as a range, both extremes are inclusive
    round_spans: Vec<(usize, usize)>,
}

impl Hook {
    pub fn parse(mut flow: impl Flow) -> Result<HookResult, HookError> {
        let first = flow.next().ok_or(Empty)?;
        let mut hook = Hook::start_with(&first)?;
        while let Some(action) = flow.next() {
            hook.perform(&action)?;
        }
        Ok(hook.finish())
    }

    fn start_with(action: &Action) -> Result<Self, HookError> {
        match action {
            MR(x) => {
                let edges: Vec<Vec<usize>> = {
                    let edges_from_root: Vec<usize> = (1..=*x).collect();
                    let ring_edges = (2..=*x).map(|i| vec![i]);
                    let mut edges = vec![edges_from_root];
                    edges.extend(ring_edges);
                    edges.push(vec![]);
                    edges
                };
                Ok(Self {
                    edges,
                    peculiar: HashMap::from([(0, Peculiarity::Root)]),
                    round_count: 0,
                    round_left: *x,
                    anchor: 1,     // 1 because root takes index 0
                    cursor: x + 1, // + 1 because root takes index 0
                    round_spans: vec![(0, *x)],
                    fastened_off: false,
                })
            }
            Ch(x) => {
                let edges: Vec<Vec<usize>> = {
                    let mut edges: Vec<Vec<usize>> = (1..*x).map(|i| vec![i]).collect();
                    edges.push(vec![]);
                    edges
                };

                let mut peculiar = HashMap::new();
                for i in 0..*x {
                    peculiar.insert(i, Peculiarity::Constrained(V::new(1.0, 0.0, 1.0)));
                }

                Ok(Self {
                    edges,
                    peculiar,
                    round_count: 0,
                    round_left: *x,
                    anchor: 0,
                    cursor: *x,
                    round_spans: vec![(0, *x - 1)],
                    fastened_off: false,
                })
            }
            _ => Err(BadStarter),
        }
    }

    fn finish(self) -> HookResult {
        HookResult::from_hook(self.edges, self.peculiar, self.round_spans)
    }

    fn edge(&mut self, i: usize) -> &mut Vec<usize> {
        if i >= self.edges.len() {
            panic!(
                "Hook malformed it's edges/nodes: {i} > {}",
                self.edges.len()
            )
        }
        &mut self.edges[i]
    }

    fn next_anchor(&mut self) {
        self.anchor += 1;
        self.round_left -= 1;
        if self.round_left == 0 {
            self.round_spans
                .push((self.cursor - self.round_count, self.cursor - 1));
            self.round_left = self.round_count;
            self.round_count = 0;
        }
    }

    fn link_to_previous_round(&mut self) {
        let current_node = self.cursor;
        self.edge(self.anchor).push(current_node);
    }

    fn link_to_previous_stitch(&mut self) {
        let current_node = self.cursor;
        self.edge(current_node - 1).push(current_node);
    }

    fn finish_stitch(&mut self) {
        self.cursor += 1;
        self.round_count += 1;
        self.edges.push(Vec::with_capacity(2));
    }

    pub fn perform(&mut self, action: &Action) -> Result<(), HookError> {
        log::trace!("Performing {action:?}");

        if self.fastened_off && !matches!(action, Goto(_)) {
            return Err(TriedToWorkAfterFastenOff);
        }

        match action {
            Sc => {
                self.link_to_previous_stitch();
                self.link_to_previous_round();
                self.finish_stitch();
                self.next_anchor();
                Ok(())
            }
            Inc => {
                for _ in 0..2 {
                    self.link_to_previous_stitch();
                    self.link_to_previous_round();
                    self.finish_stitch();
                }
                self.next_anchor();
                Ok(())
            }
            Dec => {
                for _ in 0..2 {
                    self.link_to_previous_round();
                    self.next_anchor();
                }
                self.link_to_previous_stitch();
                self.finish_stitch();
                Ok(())
            }
            Ch(_) => unimplemented!(),
            Attach(_) => unimplemented!(),
            Reverse => unimplemented!(),
            FLO => unimplemented!(),
            BLO => unimplemented!(),
            BL => unimplemented!(),
            Goto(_) => unimplemented!(),
            Mark(_) => unimplemented!(),
            MR(_) => Err(StarterInTheMiddle),
            FO => {
                self.fastened_off = true;
                self.fasten_off_with_tip()
            }
        }
    }

    fn fasten_off_with_tip(&mut self) -> Result<(), HookError> {
        assert!(
            self.round_count == 0,
            "FO for incomplete rounds is not implemented"
        );

        let (start, end) = {
            let (start, end) = self.round_spans.last().unwrap();
            (*start, end + 1)
        };

        let tip = self.cursor;
        for connected_to_tip in start..end {
            self.edge(connected_to_tip).push(tip);
        }

        self.edges.push(vec![]);
        self.round_spans.push((tip, tip));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as q;

    #[test]
    fn test_start_with_magic_ring() {
        let h = Hook::start_with(&MR(3)).unwrap();
        q!(h.anchor, 1);
        q!(h.cursor, 4);
        q!(h.round_count, 0);
        q!(h.round_left, 3);
        q!(h.round_spans.len(), 1);
        q!(h.edges, vec![vec![1, 2, 3], vec![2], vec![3], vec![],]);
    }

    #[test]
    fn test_start_with_chain() {
        let h = Hook::start_with(&Ch(3)).unwrap();
        q!(h.anchor, 0);
        q!(h.cursor, 3);
        q!(h.round_count, 0);
        q!(h.round_left, 3);
        q!(h.round_spans.len(), 1);
        q!(h.edges, vec![vec![1], vec![2], vec![]]);
    }

    #[test]
    fn test_perform_sc() {
        let mut h = Hook::start_with(&MR(6)).unwrap();
        h.perform(&Sc).unwrap();
        q!(h.anchor, 2);
        q!(h.cursor, 8);
        q!(h.round_count, 1);
        q!(h.round_left, 5);
        q!(h.round_spans, vec![(0, 6)]);

        h.perform(&Sc).unwrap();
        q!(h.anchor, 3);
        q!(h.cursor, 9);
        q!(h.round_count, 2);
        q!(h.round_left, 4);
        q!(h.round_spans, vec![(0, 6)]);
    }

    #[test]
    fn test_next_round() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.round_spans.len(), 1);
        h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3)]);
        h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3)]);
        h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3), (4, 6)]);
        q!(h.round_count, 0);
        q!(h.round_left, 3);

        h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3), (4, 6)]);
        q!(h.round_count, 1);
        q!(h.round_left, 2);
    }

    #[test]
    fn test_perform_inc() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h.perform(&Inc).unwrap();
        q!(h.anchor, 2);
        q!(h.cursor, 6);
        q!(h.round_count, 2);
        q!(h.round_left, 2);
        q!(h.round_spans, vec![(0, 3)]);
    }

    #[test]
    fn test_perform_dec() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h.perform(&Dec).unwrap();
        q!(h.anchor, 3);
        q!(h.cursor, 5);
        q!(h.round_count, 1);
        q!(h.round_left, 1);
        q!(h.round_spans, vec![(0, 3)]);
    }

    #[test]
    fn test_perform_fo_after_full_round() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.anchor, 1);
        q!(h.cursor, 4);
        q!(h.edges.len(), 4);
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        q!(h.anchor, 4);
        q!(h.cursor, 7);
        q!(h.round_count, 0);
        q!(h.round_left, 3);
        q!(h.round_spans, vec![(0, 3), (4, 6)]);
        q!(h.edges.len(), 7);
        q!(
            h.edges,
            vec![
                vec![1, 2, 3], // 0
                vec![2, 4],    // 1
                vec![3, 5],    // 2
                vec![4, 6],    // 3
                vec![5],       // 4
                vec![6],       // 5
                vec![]         //6
            ]
        );
        h.perform(&FO).unwrap();
        q!(
            h.edges,
            vec![
                vec![1, 2, 3], // 0
                vec![2, 4],    // 1
                vec![3, 5],    // 2
                vec![4, 6],    // 3
                vec![5, 7],    // 4
                vec![6, 7],    // 5
                vec![7],       // 6
                vec![]         // 7
            ]
        );
        q!(h.round_spans, vec![(0, 3), (4, 6), (7, 7)]);
    }

    #[test]
    fn test_error_on_stitch_after_fo() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h.perform(&FO).unwrap();
        h.clone().perform(&Sc).expect_err("Can't continue after FO");
        h.clone()
            .perform(&Inc)
            .expect_err("Can't continue after FO");
        h.clone()
            .perform(&Dec)
            .expect_err("Can't continue after FO");
    }
}
