use super::hook_result::{Edges, HookResult, Peculiarity};
use crate::{
    common::V,
    flow::{
        actions::{Action, Label},
        Flow,
    },
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
    DuplicateLabel(Label),
    UnknownLabel(Label),
    CantMarkAfterFO,
}

impl From<HookError> for String {
    fn from(value: HookError) -> Self {
        format!("{value:?}")
    }
}

#[derive(Clone, Debug)]
enum WorkingLoops {
    Both,
    Back,
    Front,
}

/// Span of a single generalized cylinder in the plushie
type Part = (usize, usize);

#[derive(Clone, Debug)]
struct Moment {
    anchor: usize,
    cursor: usize,
    round_count: usize,
    round_left: usize,
    working_on: WorkingLoops,
}

/// Responsible for building the graph used in the simulation
#[derive(Clone)]
pub struct Hook {
    edges: Edges,
    peculiar: HashMap<usize, Peculiarity>,
    now: Moment,
    /// Contains first and last stitch of each round. Treated as a range, both extremes are inclusive
    round_spans: Vec<(usize, usize)>,
    fastened_off: bool,
    /// Storage of index -> it's anchor
    parents: Vec<Option<usize>>,
    part_start: usize,
    parts: Vec<Part>,
    labels: HashMap<Label, Moment>,
    at_junction: bool,
    override_previous_stitch: Option<usize>,
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
                let parents: Vec<Option<usize>> = {
                    let mut tmp = vec![None];
                    tmp.append(&mut vec![Some(0); *x]);
                    tmp
                };

                Ok(Self {
                    edges,
                    peculiar: HashMap::from([(0, Peculiarity::Root)]),
                    now: Moment {
                        round_count: 0,
                        round_left: *x,
                        anchor: 1,     // 1 because root takes index 0
                        cursor: x + 1, // + 1 because root takes index 0
                        working_on: WorkingLoops::Both,
                    },
                    round_spans: vec![(0, *x)],
                    fastened_off: false,
                    parents,
                    part_start: 0,
                    parts: vec![],
                    labels: HashMap::new(),
                    at_junction: false,
                    override_previous_stitch: None,
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
                    now: Moment {
                        round_count: 0,
                        round_left: *x,
                        anchor: 0,
                        cursor: *x,
                        working_on: WorkingLoops::Both,
                    },
                    round_spans: vec![(0, *x - 1)],
                    fastened_off: false,
                    parents: vec![None; *x],
                    part_start: 0,
                    parts: vec![],
                    labels: HashMap::new(),
                    at_junction: false,
                    override_previous_stitch: None,
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
        self.now.anchor += 1;
        self.now.round_left -= 1;
        if self.now.round_left == 0 {
            self.round_spans
                .push((self.now.cursor - self.now.round_count, self.now.cursor - 1));
            self.now.round_left = self.now.round_count;
            if self.at_junction {
                self.now.anchor = self.now.cursor - self.now.round_count;
                self.at_junction = false;
            }
            self.now.round_count = 0;
        }
    }

    fn link_to_previous_round(&mut self) {
        let current_node = self.now.cursor;
        self.edge(self.now.anchor).push(current_node);
    }

    fn link_to_previous_stitch(&mut self) {
        let cursor_for_borrow_checker = self.now.cursor;
        let previous_node = match self.override_previous_stitch {
            Some(x) => {
                self.override_previous_stitch = None;
                x
            }
            None => self.now.cursor - 1,
        };
        self.edge(previous_node).push(cursor_for_borrow_checker);
    }

    fn handle_working_loop(&mut self) {
        if matches!(self.now.working_on, WorkingLoops::Both) {
            return;
        }

        let mother = self.now.anchor;
        let father = self.now.anchor + 1;
        let grandparent = self.parents[self.now.anchor].expect("Grandparent exists");
        let points_on_push_plane = (father, mother, grandparent);
        match self.now.working_on {
            WorkingLoops::Both => unreachable!(),
            WorkingLoops::Back => self
                .peculiar
                .insert(self.now.cursor, Peculiarity::BLO(points_on_push_plane))
                .map_or((), |_| panic!("Multi-peculiarity")),
            WorkingLoops::Front => self
                .peculiar
                .insert(self.now.cursor, Peculiarity::FLO(points_on_push_plane))
                .map_or((), |_| panic!("Multi-peculiarity")),
        };
    }

    fn finish_stitch(&mut self) {
        self.edges.push(Vec::with_capacity(2));
        self.parents.push(Some(self.now.anchor));
        self.handle_working_loop();
        self.now.cursor += 1;
        self.now.round_count += 1;
    }

    fn restore(&mut self, label: Label) -> Result<(), HookError> {
        let mut moment = self.labels.get(&label).ok_or(UnknownLabel(label))?.clone();
        self.override_previous_stitch = Some(moment.cursor - 1);
        moment.cursor = self.now.cursor;
        self.now = moment;
        self.at_junction = true;
        self.fastened_off = false;
        Ok(())
    }

    fn save(&mut self, label: Label) -> Result<(), HookError> {
        if self.fastened_off {
            return Err(CantMarkAfterFO);
        }
        if let Some(_) = self.labels.insert(label, self.now.clone()) {
            return Err(DuplicateLabel(label));
        }
        Ok(())
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
            }
            Inc => {
                for _ in 0..2 {
                    self.link_to_previous_stitch();
                    self.link_to_previous_round();
                    self.finish_stitch();
                }
                self.next_anchor();
            }
            Dec => {
                self.link_to_previous_round();
                self.next_anchor();
                self.link_to_previous_round();
                self.link_to_previous_stitch();
                self.finish_stitch();
                self.next_anchor();
            }
            Ch(_) => unimplemented!(),
            Attach(_) => unimplemented!(),
            Reverse => unimplemented!(),
            FLO => self.now.working_on = WorkingLoops::Front,
            BLO => self.now.working_on = WorkingLoops::Back,
            BL => self.now.working_on = WorkingLoops::Both,
            Goto(label) => self.restore(*label)?,
            Mark(label) => self.save(*label)?,
            MR(_) => return Err(StarterInTheMiddle),
            FO => {
                self.fastened_off = true;
                self.fasten_off_with_tip()?
            }
        };
        Ok(())
    }

    fn fasten_off_with_tip(&mut self) -> Result<(), HookError> {
        assert!(
            self.now.round_count == 0,
            "FO for incomplete rounds is not implemented"
        );

        let (start, end) = {
            let (start, end) = self.round_spans.last().unwrap();
            (*start, end + 1)
        };

        let tip = self.now.cursor;
        for connected_to_tip in start..end {
            self.edge(connected_to_tip).push(tip);
        }

        self.edges.push(vec![]);
        self.round_spans.push((tip, tip));
        self.parts.push((self.part_start, tip));
        self.now.cursor += 1;
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
        q!(h.now.anchor, 1);
        q!(h.now.cursor, 4);
        q!(h.now.round_count, 0);
        q!(h.now.round_left, 3);
        q!(h.round_spans.len(), 1);
        q!(h.edges, vec![vec![1, 2, 3], vec![2], vec![3], vec![],]);
    }

    #[test]
    fn test_start_with_chain() {
        let h = Hook::start_with(&Ch(3)).unwrap();
        q!(h.now.anchor, 0);
        q!(h.now.cursor, 3);
        q!(h.now.round_count, 0);
        q!(h.now.round_left, 3);
        q!(h.round_spans.len(), 1);
        q!(h.edges, vec![vec![1], vec![2], vec![]]);
    }

    #[test]
    fn test_perform_sc() {
        let mut h = Hook::start_with(&MR(6)).unwrap();
        h.perform(&Sc).unwrap();
        q!(h.now.anchor, 2);
        q!(h.now.cursor, 8);
        q!(h.now.round_count, 1);
        q!(h.now.round_left, 5);
        q!(h.round_spans, vec![(0, 6)]);

        h.perform(&Sc).unwrap();
        q!(h.now.anchor, 3);
        q!(h.now.cursor, 9);
        q!(h.now.round_count, 2);
        q!(h.now.round_left, 4);
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
        q!(h.now.round_count, 0);
        q!(h.now.round_left, 3);

        h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3), (4, 6)]);
        q!(h.now.round_count, 1);
        q!(h.now.round_left, 2);
    }

    #[test]
    fn test_perform_inc() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h.perform(&Inc).unwrap();
        q!(h.now.anchor, 2);
        q!(h.now.cursor, 6);
        q!(h.now.round_count, 2);
        q!(h.now.round_left, 2);
        q!(h.round_spans, vec![(0, 3)]);
    }

    #[test]
    fn test_perform_dec() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h.perform(&Dec).unwrap();
        q!(h.now.anchor, 3);
        q!(h.now.cursor, 5);
        q!(h.now.round_count, 1);
        q!(h.now.round_left, 1);
        q!(h.round_spans, vec![(0, 3)]);
    }

    #[test]
    fn test_perform_fo_after_full_round() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.now.anchor, 1);
        q!(h.now.cursor, 4);
        q!(h.edges.len(), 4);
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        q!(h.now.anchor, 4);
        q!(h.now.cursor, 7);
        q!(h.now.round_count, 0);
        q!(h.now.round_left, 3);
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
    fn test_round_spans_with_dec() {
        let mut h = Hook::start_with(&MR(4)).unwrap();
        h.perform(&Dec).unwrap();
        h.perform(&Dec).unwrap();
        assert_eq!(h.round_spans, vec![(0, 4), (5, 6)]);
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

    #[test]
    fn test_multipart() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h.perform(&Mark(0)).unwrap();
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3), (4, 6)]);
        q!(
            h.edges,
            vec![
                vec![1, 2, 3], // 0
                vec![2, 4],    // 1
                vec![3, 5],    // 2
                vec![4, 6],    // 3
                vec![5],       // 4
                vec![6],       // 5
                vec![]         // 6
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
        h.perform(&Goto(0)).unwrap();
        q!(h.now.cursor, 8);
        q!(h.now.anchor, 1);
        q!(h.override_previous_stitch, Some(3));
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        h.perform(&Sc).unwrap();
        q!(
            h.edges,
            vec![
                vec![1, 2, 3],     // 0 - root
                vec![2, 4, 8],     // 1 - ring
                vec![3, 5, 9],     // 2 - ring
                vec![4, 6, 8, 10], // 3 - ring
                vec![5, 7],        // 4 - sc
                vec![6, 7],        // 5 - sc
                vec![7],           // 6 - sc
                vec![],            // 7 - tip 1
                vec![9],           // 8 - sc
                vec![10],          // 9 - sc
                vec![],            // 10 - sc
            ]
        );
    }
}
