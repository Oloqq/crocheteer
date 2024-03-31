mod starters;
mod state_mgmt;
mod utils;
mod working_stitch;

use self::utils::*;
use self::working_stitch::Stitch;
use self::HookError::*;
use super::hook_result::{Edges, HookResult};
use crate::{
    flow::{
        actions::{Action, Label},
        Flow,
    },
    sanity,
};
use std::collections::{HashMap, HashSet, VecDeque as Queue};

/// Span of a single generalized cylinder in the plushie
type Part = (usize, usize);

#[derive(Clone, Debug)]
struct Moment {
    cursor: usize,
    anchors: Queue<usize>,
    round_count: usize,
    round_left: usize,
    working_on: WorkingLoops,
}

/// Responsible for building the graph used in the simulation
#[derive(Clone, Debug)]
pub struct Hook {
    edges: Edges,
    peculiar: HashMap<usize, Peculiarity>,
    now: Moment,
    /// Contains first and last stitch of each round. Treated as a range, both extremes are inclusive
    /// When chains are introduced, round_spans acts merely as data for Initializer::Cylinder,
    /// and it's content may not be connected to what a human would consider a working round
    round_spans: Vec<(usize, usize)>,
    fastened_off: bool,
    /// Storage of index -> it's anchor
    parents: Vec<Option<usize>>,
    part_start: usize,
    parts: Vec<Part>,
    labels: HashMap<Label, Moment>,
    at_junction: bool,
    override_previous_stitch: Option<usize>,
    color: Color,
    colors: Vec<Color>,
}

fn is_uniq(vec: &Vec<Point>) -> bool {
    let uniq = vec
        .into_iter()
        .map(|v| format!("{:?}", v.coords))
        .collect::<HashSet<_>>();
    uniq.len() == vec.len()
}

impl Hook {
    pub fn parse(mut flow: impl Flow) -> Result<HookResult, HookError> {
        let first = flow.next().ok_or(Empty)?;
        let mut hook = Hook::start_with(&first)?;
        while let Some(action) = flow.next() {
            hook = hook.perform(&action)?;
        }

        let result = hook.finish();
        sanity!(assert!(
            is_uniq(&result.nodes),
            "hook created duplicate positions"
        ));
        log::debug!("edges: {:?}, len: {}", result.edges, result.edges.len());
        log::debug!("nodes len: {}", result.nodes.len());
        Ok(result)
    }

    fn finish(mut self) -> HookResult {
        self.edges.cleanup();
        HookResult::from_hook(self.edges, self.peculiar, self.round_spans, self.colors)
    }

    pub fn perform(mut self, action: &Action) -> Result<Self, HookError> {
        log::trace!("Performing {action:?}");

        if self.fastened_off && !matches!(action, Goto(_)) {
            return Err(TriedToWorkAfterFastenOff);
        }

        match action {
            Sc => {
                self = Stitch::linger(self)?
                    .pull_through()?
                    .pull_over()?
                    .finish()?;
            }
            Inc => {
                self = Stitch::linger(self)?
                    .pull_through()?
                    .pull_over()?
                    .pull_through()?
                    .pull_over()?
                    .finish()?;
            }
            Dec => {
                self = Stitch::linger(self)?
                    .pull_through()?
                    .next_anchor()?
                    .pull_through()?
                    .pull_over()?
                    .finish()?;
            }
            Ch(x) => {
                let start = self.now.cursor;
                for _ in 0..*x {
                    self = Stitch::linger(self)?.pull_over()?.finish()?;
                }
                self.round_spans.push((start, self.now.cursor - 1));
            }
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
                self = Stitch::fasten_off_with_tip(self)
            }
            Color(c) => self.color = *c,
        };
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as q;

    #[test]
    fn test_start_with_magic_ring() {
        let h = Hook::start_with(&MR(3)).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        q!(h.now.cursor, 4);
        q!(h.now.round_count, 0);
        q!(h.now.round_left, 3);
        q!(h.round_spans.len(), 1);
        q!(
            h.edges,
            Edges::from_unchecked(vec![vec![], vec![0], vec![0, 1], vec![0, 2], vec![]])
        );
    }

    #[test]
    fn test_start_with_chain() {
        let h = Hook::start_with(&Ch(3)).unwrap();
        q!(h.now.anchors, Queue::from([0, 1, 2]));
        q!(h.now.cursor, 3);
        q!(h.now.round_count, 0);
        q!(h.now.round_left, 3);
        q!(h.round_spans.len(), 1);
        q!(h.edges, Edges::from(vec![vec![1], vec![2], vec![], vec![]]));
        q!(
            h.edges,
            Edges::from_unchecked(vec![vec![], vec![0], vec![1], vec![]])
        );
    }

    #[test]
    fn test_perform_sc() {
        let mut h = Hook::start_with(&MR(6)).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3, 4, 5, 6]));
        h = h.perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from([2, 3, 4, 5, 6, 7]));
        q!(h.now.cursor, 8);
        q!(h.now.round_count, 1);
        q!(h.now.round_left, 5);
        q!(h.round_spans, vec![(0, 6)]);

        h = h.perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from([3, 4, 5, 6, 7, 8]));
        q!(h.now.cursor, 9);
        q!(h.now.round_count, 2);
        q!(h.now.round_left, 4);
        q!(h.round_spans, vec![(0, 6)]);
    }

    #[test]
    fn test_next_round() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.round_spans.len(), 1);
        h = h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3)]);
        h = h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3)]);
        h = h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3), (4, 6)]);
        q!(h.now.round_count, 0);
        q!(h.now.round_left, 3);

        h = h.perform(&Sc).unwrap();
        q!(h.round_spans, vec![(0, 3), (4, 6)]);
        q!(h.now.round_count, 1);
        q!(h.now.round_left, 2);
    }

    #[test]
    fn test_perform_inc() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        h = h.perform(&Inc).unwrap();
        q!(h.now.anchors, Queue::from([2, 3, 4, 5]));
        q!(h.now.cursor, 6);
        q!(h.now.round_count, 2);
        q!(h.now.round_left, 2);
        q!(h.round_spans, vec![(0, 3)]);
    }

    #[test]
    fn test_perform_dec() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        h = h.perform(&Dec).unwrap();
        q!(h.now.anchors, Queue::from([3, 4]));
        q!(h.now.cursor, 5);
        q!(h.now.round_count, 1);
        q!(h.now.round_left, 1);
        q!(h.round_spans, vec![(0, 3)]);
    }

    #[test]
    fn test_perform_fo_after_full_round() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        q!(h.now.cursor, 4);
        q!(h.edges.len(), 5);
        h = h.perform(&Sc).unwrap();
        h = h.perform(&Sc).unwrap();
        h = h.perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from([4, 5, 6]));
        q!(h.now.cursor, 7);
        q!(h.now.round_count, 0);
        q!(h.now.round_left, 3);
        q!(h.round_spans, vec![(0, 3), (4, 6)]);
        q!(h.edges.len(), 8);
        q!(
            h.edges,
            Edges::from(vec![
                vec![1, 2, 3], // 0
                vec![2, 4],    // 1
                vec![3, 5],    // 2
                vec![4, 6],    // 3
                vec![5],       // 4
                vec![6],       // 5
                vec![],        //6
                vec![]
            ])
        );
        h = h.perform(&FO).unwrap();
        q!(h.now.anchors, Queue::from([]));
        q!(
            h.edges,
            Edges::from(vec![
                vec![1, 2, 3], // 0
                vec![2, 4],    // 1
                vec![3, 5],    // 2
                vec![4, 6],    // 3
                vec![5, 7],    // 4
                vec![6, 7],    // 5
                vec![7],       // 6
                vec![],        // 7
                vec![]
            ])
        );
        q!(h.round_spans, vec![(0, 3), (4, 6), (7, 7)]);
    }

    #[test]
    fn test_round_spans_with_dec() {
        let mut h = Hook::start_with(&MR(4)).unwrap();
        h = h.perform(&Dec).unwrap();
        h = h.perform(&Dec).unwrap();
        assert_eq!(h.round_spans, vec![(0, 4), (5, 6)]);
    }

    #[test]
    fn test_error_on_stitch_after_fo() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h = h.perform(&FO).unwrap();
        h.clone().perform(&Sc).expect_err("Can't continue after FO");
        h.clone()
            .perform(&Inc)
            .expect_err("Can't continue after FO");
        h.clone()
            .perform(&Dec)
            .expect_err("Can't continue after FO");
    }

    #[test]
    fn test_goto_after_fo() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        h = h.perform(&Mark(0)).unwrap();
        h = h.perform(&Sc).unwrap();
        h = h.perform(&Sc).unwrap();
        h = h.perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from([4, 5, 6]));
        q!(h.round_spans, vec![(0, 3), (4, 6)]);
        q!(
            h.edges,
            Edges::from(vec![
                vec![1, 2, 3], // 0
                vec![2, 4],    // 1
                vec![3, 5],    // 2
                vec![4, 6],    // 3
                vec![5],       // 4
                vec![6],       // 5
                vec![],        // 6
                vec![]
            ])
        );
        h = h.perform(&FO).unwrap();
        q!(
            h.edges,
            Edges::from(vec![
                vec![1, 2, 3], // 0
                vec![2, 4],    // 1
                vec![3, 5],    // 2
                vec![4, 6],    // 3
                vec![5, 7],    // 4
                vec![6, 7],    // 5
                vec![7],       // 6
                vec![],        // 7
                vec![]
            ])
        );
        q!(h.round_spans, vec![(0, 3), (4, 6), (7, 7)]);
        q!(h.now.anchors, Queue::from([]));
        h = h.perform(&Goto(0)).unwrap();
        q!(h.now.cursor, 8);
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        q!(h.override_previous_stitch, Some(3));
        h = h.perform(&Sc).unwrap();
        h = h.perform(&Sc).unwrap();
        h = h.perform(&Sc).unwrap();
        q!(
            h.edges,
            Edges::from(vec![
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
                vec![],
            ])
        );
    }

    #[test]
    fn test_chain_simple() {
        let mut h = Hook::start_with(&MR(3)).unwrap();
        h = h.perform(&Ch(3)).unwrap();
        q!(
            h.edges,
            Edges::from(vec![
                vec![1, 2, 3],
                vec![2],
                vec![3],
                vec![4],
                vec![5],
                vec![6],
                vec![],
                vec![],
            ])
        )
    }
}
