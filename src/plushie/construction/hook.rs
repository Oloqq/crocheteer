use super::graph::{Edges, Graph, Peculiarity};
use crate::flow::actions::Action;
use Action::*;
use HookError::*;

use std::collections::HashMap;

#[derive(Debug)]
pub enum HookError {
    BadStarter,
    StarterInTheMiddle,
    ChainStart,
}

impl From<HookError> for String {
    fn from(value: HookError) -> Self {
        format!("{value:?}")
    }
}

// chains shall be approximated as a line from start point to attachment point
// how to avoid mutiple shoves of the nodes array during construction (e.g. with multiple FOs that should be placed at the beginning)
// constraints need to stay alive, otherwise tips get fucked up

/// Responsible for building the graph used in the simulation
pub struct Hook {
    edges: Edges,
    peculiar: HashMap<usize, Peculiarity>,
    round_count: usize,
    round_left: usize,
    anchor: usize,
    next: usize,
    round_starts: Vec<usize>,
}

impl Hook {
    pub fn start_with(action: &Action) -> Result<Self, HookError> {
        if !action.is_starter() {
            return Err(BadStarter);
        }

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
                let peculiar = HashMap::from([(0, Peculiarity::Root)]);
                Ok(Self {
                    edges,
                    peculiar,
                    round_count: *x,
                    round_left: 0,
                    anchor: 1,
                    next: x + 1, // + 1 because root takes index 0
                    round_starts: vec![],
                })
            }
            _ => Err(BadStarter),
        }
    }

    pub fn finish(self) -> Graph {
        Graph::from_hook(self.edges, self.peculiar, self.round_starts)
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

    pub fn perform(&mut self, action: &Action) -> Result<(), HookError> {
        match action {
            Sc => {
                if self.round_left == 0 {
                    self.round_starts.push(self.next);
                    self.round_left = self.round_count;
                    self.round_count = 0;
                }
                let this = self.next;
                self.edge(self.anchor).push(this);
                self.edge(this - 1).push(this);
                self.edges.push(vec![]);
                self.round_count += 1;
                self.round_left -= 1;
                self.next += 1;
                self.anchor += 1;
                Ok(())
            }
            Inc => {
                for _ in 0..2 {
                    if self.round_left == 0 {
                        self.round_starts.push(self.next);
                        self.round_left = self.round_count;
                        self.round_count = 0;
                    }
                    let this = self.next;
                    self.edge(self.anchor).push(this);
                    self.edge(this - 1).push(this);
                    self.edges.push(vec![]);
                    self.round_count += 1;
                    self.next += 1;
                }
                self.round_left -= 1;
                self.anchor += 1;
                Ok(())
            }
            Dec => {
                let this = self.next;
                for _ in 0..2 {
                    if self.round_left == 0 {
                        self.round_starts.push(self.next);
                        self.round_left = self.round_count;
                        self.round_count = 0;
                    }
                    self.edge(self.anchor).push(this);
                    self.round_left -= 1;
                    self.anchor += 1;
                }
                self.edge(this - 1).push(this);
                self.edges.push(vec![]);
                self.round_count += 1;
                self.next += 1;
                Ok(())
            }
            Ch(_) => unimplemented!(),
            Attach(_) => unimplemented!(),
            Reverse => unimplemented!(),
            FLO => unimplemented!(),
            BLO => unimplemented!(),
            Both => unimplemented!(),
            Goto(_) => unimplemented!(),
            Mark(_) => unimplemented!(),
            MR(_) => Err(StarterInTheMiddle),
            FO => {
                let this = self.next;
                let i = self.next - 1;
                for di in 0..self.round_count {
                    self.edge(i - di).push(this);
                }
                self.edges.push(vec![]);
                // TODO set self.next to Option::None
                Ok(())
            }
        }
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
        q!(h.next, 4);
        q!(h.round_count, 3);
        q!(h.round_left, 0);
        q!(h.round_starts.len(), 0);
        q!(h.edges, vec![vec![1, 2, 3], vec![2], vec![3], vec![],]);
    }

    #[test]
    fn test_perform_sc() {
        let mut h = Hook::start_with(&MR(6)).unwrap();
        h.perform(&Sc).unwrap();
        q!(h.anchor, 2);
        q!(h.next, 8);
        q!(h.round_count, 1);
        q!(h.round_left, 5);
        q!(h.round_starts, vec![7]);

        h.perform(&Sc).unwrap();
        q!(h.anchor, 3);
        q!(h.next, 9);
        q!(h.round_count, 2);
        q!(h.round_left, 4);
        q!(h.round_starts, vec![7]);
    }

    #[test]
    fn test_next_round() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.round_starts.len(), 0);
        h.perform(&Sc).unwrap();
        q!(h.round_starts, vec![4]);
        h.perform(&Sc).unwrap();
        q!(h.round_starts, vec![4]);
        h.perform(&Sc).unwrap();
        q!(h.round_starts, vec![4]);
        q!(h.round_count, 3);
        q!(h.round_left, 0);

        h.perform(&Sc).unwrap();
        q!(h.round_starts, vec![4, 7]);
        q!(h.round_count, 1);
        q!(h.round_left, 2);
    }

    #[test]
    fn test_perform_inc() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h.perform(&Inc).unwrap();
        q!(h.anchor, 2);
        q!(h.next, 6);
        q!(h.round_count, 2);
        q!(h.round_left, 2);
        q!(h.round_starts, vec![4]);
    }

    #[test]
    fn test_perform_dec() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h.perform(&Dec).unwrap();
        q!(h.anchor, 3);
        q!(h.next, 5);
        q!(h.round_count, 1);
        q!(h.round_left, 1);
        q!(h.round_starts, vec![4]);
    }

    #[test]
    fn test_perform_fo_after_full_round() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.anchor, 1);
        q!(h.next, 4);
        q!(h.edges.len(), 4);
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        q!(h.anchor, 4);
        q!(h.next, 7);
        q!(h.round_count, 3);
        q!(h.round_left, 0);
        q!(h.round_starts, vec![4]);
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
        // TODO assert next Sc won't succeed
    }

    #[test]
    #[ignore = "not yet"]
    fn test_perform_fo_after_unfinished_round() {
        todo!()
    }

    // #[test]
    // fn test_adding_edges_with_inc() {
    //     todo!()
    // }

    // #[test]
    // fn test_adding_edges_with_dec() {
    //     todo!()
    // }
}
