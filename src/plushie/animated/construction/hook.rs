pub mod leniency;
mod starters;
mod state_mgmt;
mod utils;
mod working_stitch;

use leniency::Leniency;

use self::utils::*;
use self::working_stitch::Stitch;
use self::HookError::*;
use super::hook_result::{Edges, HookResult};
use crate::{
    acl::{
        actions::{Action, Label},
        Flow,
    },
    sanity,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Moment {
    /// Node index to be created
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
    /// Contains first and last stitch of each round. Treated as a range, both extremes are inclusive.
    round_spans: Vec<(usize, usize)>,
    /// Storage of index -> it's anchor, used for single loop forces
    parents: Vec<Option<usize>>,
    /// Storage of spots for Mark and Goto
    labels: HashMap<Label, Moment>,
    /// Current color/yarn. Not stored in Moment as typically yarn changes happpen independently of switching positions.
    color: colors::Color,
    /// Storage of index -> it's color. todo: use less memory by storing changes
    colors: Vec<colors::Color>,
    // Previous stitch might need to be overwritten after a Goto
    override_previous_stitch: Option<usize>,
    /// Leniency policy, may allow recovery after `HookError`s. Some leniency is beneficial with genetic algorithms
    leniency: Leniency,

    /// Last stitch created (not counting actions like mark, goto)
    last_stitch: Option<Action>,
    /// Was the last action a mark?
    last_mark: Option<Action>,
}

fn is_uniq(vec: &Vec<Point>) -> bool {
    let uniq = vec
        .into_iter()
        .map(|v| format!("{:?}", v.coords))
        .collect::<HashSet<_>>();
    uniq.len() == vec.len()
}

fn split_moment(
    source: &mut Moment,
    attachment_anchor: usize,
    new_anchors: Vec<usize>,
) -> (Moment, Moment, (usize, usize)) {
    let new_span = (source.cursor - source.round_count, source.cursor - 1);
    let attachment_i = source
        .anchors
        .iter()
        .position(|x| *x == attachment_anchor)
        .expect("attachment anchor present in current ring"); // TODO real error handling
    let mut ring_a = source.anchors.split_off(attachment_i);
    source.anchors.extend(new_anchors.iter().rev());
    let ring_b = &source.anchors;

    ring_a.pop_front();
    let mut new_anchors = new_anchors;
    new_anchors.pop();
    ring_a.append(&mut new_anchors.into());
    let moment_a = Moment {
        round_count: 0,
        round_left: ring_a.len(),
        cursor: source.cursor,
        anchors: ring_a,
        working_on: WorkingLoops::Both,
    };

    let moment_b = Moment {
        round_count: 0,
        round_left: ring_b.len(),
        cursor: source.cursor,
        anchors: ring_b.clone(),
        working_on: WorkingLoops::Both,
    };

    (moment_a, moment_b, new_span)
}

impl Hook {
    pub fn parse(mut flow: impl Flow, _leniency: &Leniency) -> Result<HookResult, HookError> {
        if flow.peek().is_none() {
            return Err(Empty);
        }
        let mut hook = Hook::from_starting_sequence(&mut flow)?;
        let mut i: u32 = 0;
        while let Some(action) = flow.next() {
            log::trace!("Performing [{i}] {action:?}");
            i += 1;
            hook = hook.perform(&action)?;
        }

        let result = hook.finish();
        sanity!(assert!(
            is_uniq(&result.nodes),
            "hook created duplicate positions"
        ));
        Ok(result)
    }

    fn finish(mut self) -> HookResult {
        self.edges.cleanup();
        HookResult::from_hook(self.edges, self.peculiar, self.round_spans, self.colors)
    }

    fn do_perform(mut self, action: &Action) -> Result<Self, HookError> {
        match action {
            Sc => {
                self = Stitch::linger(self)?
                    .pull_through()?
                    .pull_over()?
                    .finish()?
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
                if matches!(self.last_stitch, Some(Ch(_))) {
                    return Err(ChainAfterChain);
                }
                self = Stitch::linger(self)?.chain(*x)?;
            }
            Attach(label, chain_size) => {
                // create a chain
                // attach it to given point
                // assumption: user marked a spot X right before an attach
                // after attaching, the plushie splits into 2 working rings: A and B
                // A is the one that user will work when doing rounds normally
                // ring B can be accessed by goto(X)

                let starting_anchor = self.now.cursor;
                let attachment_anchor = self.labels.get(label).unwrap().cursor - 1;
                let new_anchors: Vec<usize>;
                (new_anchors, self) =
                    Stitch::linger(self)?.attaching_chain(*chain_size, attachment_anchor)?;
                let mut moment_b;
                (self, moment_b) = self.split_moment(attachment_anchor, new_anchors);
                // let ring_b = self.split_current_moment(attaching_anchor, new_anchors);

                if let Some(Mark(ring_b_label)) = self.last_mark {
                    assert!(self.labels.contains_key(&ring_b_label));
                    moment_b.cursor = starting_anchor;
                    self.labels.insert(ring_b_label, moment_b);
                }
            }
            Reverse => unimplemented!(),
            FLO => self.now.working_on = WorkingLoops::Front,
            BLO => self.now.working_on = WorkingLoops::Back,
            BL => self.now.working_on = WorkingLoops::Both,
            Goto(label) => self.restore(*label)?,
            Mark(label) => self.save(*label)?,
            MR(_) | MRLabeled(..) => return Err(StarterInTheMiddle),
            FO => self = Stitch::fasten_off_with_tip(self)?,
            Color(c) => self.color = *c,
        };

        match action {
            Reverse | FLO | BLO | BL | Goto(_) | FO | Action::Color(_) => self.last_mark = None,
            Mark(_) => self.last_mark = Some(*action),
            _ => {
                self.last_stitch = Some(*action);
                self.last_mark = None
            }
        }

        Ok(self)
    }

    pub fn perform(self, action: &Action) -> Result<Self, HookError> {
        match self.leniency {
            Leniency::NoMercy => self.do_perform(action),
            Leniency::SkipIncorrect => {
                // If this approach turns out to be actually useful, a more efficient implementation is necessary
                let copy = self.clone();
                match copy.do_perform(action) {
                    Ok(hook) => Ok(hook),
                    Err(_) => Ok(self),
                }
            }
            Leniency::GeneticFixups => todo!(),
        }
    }

    fn split_moment(mut self, attachment_anchor: usize, new_anchors: Vec<usize>) -> (Self, Moment) {
        let (moment_a, moment_b, new_span) =
            split_moment(&mut self.now, attachment_anchor, new_anchors);
        log::debug!("Pushing round_span: {new_span:?}");
        self.round_spans.push(new_span);
        self.now = moment_a;
        // self = Stitch::linger(self)
        //     .unwrap()
        //     .pull_through()
        //     .unwrap()
        //     .pull_over()
        //     .unwrap()
        //     .finish()
        //     .unwrap();
        (self, moment_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as q;

    const COLOR: colors::Color = colors::RED;

    #[test]
    fn test_start_with_magic_ring() {
        let h = Hook::start_with(&MR(3), COLOR).unwrap();
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
        let h = Hook::start_with(&Ch(3), COLOR).unwrap();
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
        let mut h = Hook::start_with(&MR(6), COLOR).unwrap();
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
        let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
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
        let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
        h = h.perform(&Inc).unwrap();
        q!(h.now.anchors, Queue::from([2, 3, 4, 5]));
        q!(h.now.cursor, 6);
        q!(h.now.round_count, 2);
        q!(h.now.round_left, 2);
        q!(h.round_spans, vec![(0, 3)]);
        q!(
            h.edges,
            Edges::from_unchecked(vec![
                vec![],
                vec![0],
                vec![0, 1],
                vec![0, 2],
                vec![3, 1],
                vec![4, 1],
                vec![]
            ])
        )
    }

    #[test]
    fn test_perform_dec() {
        let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
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
        let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
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
        let mut h = Hook::start_with(&MR(4), COLOR).unwrap();
        h = h.perform(&Dec).unwrap();
        h = h.perform(&Dec).unwrap();
        assert_eq!(h.round_spans, vec![(0, 4), (5, 6)]);
    }

    #[test]
    fn test_error_on_stitch_after_fo() {
        let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
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
        let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
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
        let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
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
        );
    }

    #[test]
    fn test_attach1() {
        let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
        let attach_here = 0;
        let return_here = 1;
        h = h.perform(&Mark(attach_here)).unwrap();
        q!(h.now.anchors, Queue::from(vec![1, 2, 3]));
        q!(h.now.round_count, 0);
        h = h.perform(&Sc).unwrap();
        h = h.perform(&Mark(return_here)).unwrap();
        q!(h.now.anchors, Queue::from(vec![2, 3, 4]));
        q!(h.now.round_count, 1);
        q!(h.now.round_left, 2);
        q!(
            h.edges,
            Edges::from(vec![
                vec![],     // 0: root
                vec![0],    // 1: mr 1
                vec![0, 1], // 2: mr 2
                vec![0, 2], // 3: mr 3, mark
                vec![1, 3], // 4: sc
                vec![],
            ])
        );
        h = h.perform(&Attach(attach_here, 3)).unwrap();
        q!(
            h.edges,
            Edges::from(vec![
                vec![],     // 0: root
                vec![0],    // 1: mr 1
                vec![0, 1], // 2: mr 2
                vec![0, 2], // 3: mr 3, mark
                vec![1, 3], // 4: sc 1
                vec![4],    // 5: ch 1
                vec![5],    // 6: ch 2
                vec![6],    // 7: ch 3
                vec![3, 7], // 8: attaching
                vec![],
            ])
        );
        let part_a = h.now;
        let part_b = h.labels.get(&return_here).unwrap();

        q!(part_a.anchors, Queue::from(vec![4, 5, 6, 7]));
        q!(part_a.round_count, 0);
        q!(part_a.round_left, 4);

        q!(part_b.anchors, Queue::from(vec![2, 8, 7, 6, 5]));
        q!(part_b.round_count, 0);
        q!(part_b.round_left, 5);
    }

    #[test]
    fn test_sc_after_attach() {
        let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
        let attach_here = 0;
        let return_here = 1;
        h = h.perform(&Mark(attach_here)).unwrap();
        q!(h.now.anchors, Queue::from(vec![1, 2, 3]));
        q!(h.now.round_count, 0);
        h = h.perform(&Sc).unwrap();
        h = h.perform(&Mark(return_here)).unwrap();
        q!(h.now.anchors, Queue::from(vec![2, 3, 4]));
        q!(h.now.round_count, 1);
        q!(h.now.round_left, 2);
        q!(
            h.edges,
            Edges::from(vec![
                vec![],     // 0: root
                vec![0],    // 1: mr 1
                vec![0, 1], // 2: mr 2
                vec![0, 2], // 3: mr 3, mark
                vec![1, 3], // 4: sc
                vec![],
            ])
        );
        h = h.perform(&Attach(attach_here, 3)).unwrap();
        q!(
            h.edges,
            Edges::from(vec![
                vec![],     // 0: root
                vec![0],    // 1: mr 1
                vec![0, 1], // 2: mr 2
                vec![0, 2], // 3: mr 3, mark
                vec![1, 3], // 4: sc 1
                vec![4],    // 5: ch 1
                vec![5],    // 6: ch 2
                vec![6],    // 7: ch 3
                vec![3, 7], // 8
                vec![],
            ])
        );
        {
            let part_a = &h.now;

            q!(part_a.anchors, Queue::from(vec![4, 5, 6, 7]));
            q!(part_a.round_count, 0);
            q!(part_a.round_left, 4);
        }

        h = h.perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from(vec![5, 6, 7, 9]));
        q!(h.now.round_count, 1);
        q!(h.now.round_left, 3);

        q!(
            h.edges,
            Edges::from(vec![
                vec![],     // 0: root
                vec![0],    // 1: mr 1
                vec![0, 1], // 2: mr 2
                vec![0, 2], // 3: mr 3, mark
                vec![1, 3], // 4: sc 1
                vec![4],    // 5: ch 1
                vec![5],    // 6: ch 2
                vec![6],    // 7: ch 3
                vec![3, 7], // 8: attaching
                vec![4, 8], // 9: sc
                vec![],
            ])
        );

        h = h.perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from(vec![6, 7, 9, 10]));
        h = h.perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from(vec![7, 9, 10, 11]));
        h = h.perform(&Sc).unwrap();
        h = h.perform(&Sc).unwrap();

        let result = h.finish();
        q!(result.nodes.len(), result.colors.len());
    }

    #[test]
    fn test_split_moment() {
        let mut source = Moment {
            cursor: 20,
            anchors: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].into(),
            round_count: 0,
            round_left: 10,
            working_on: WorkingLoops::Both,
        };
        let (moment_a, moment_b, _new_span) = split_moment(&mut source, 6, [13, 14, 15, 16].into());
        println!("{:?} {:?}", moment_a.anchors, moment_b.anchors);
        q!(moment_a.anchors.len(), 9);
        q!(moment_b.anchors.len(), 9);
    }

    #[test]
    fn test_starting_from_color() {
        let mut flow =
            crate::acl::simple_flow::SimpleFlow::new(vec![Color(colors::RED), MR(3), Ch(3)]);
        let mut h = Hook::from_starting_sequence(&mut flow).unwrap();
        h = h.perform(&flow.next().unwrap()).unwrap();
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
        );
    }

    // #[test]
    // fn test_multipart_start() {
    //     let mut h = Hook::start_with(&MR(3)).unwrap();
    //     let attach_here = 0;
    //     let return_here = 1;
    //     h = h.perform(&Mark(attach_here)).unwrap();
    //     q!(h.now.anchors, Queue::from(vec![1, 2, 3]));
    //     q!(h.now.round_count, 0);
    //     h = h.perform(&Sc).unwrap();
    //     h = h.perform(&Mark(return_here)).unwrap();
    //     q!(h.now.anchors, Queue::from(vec![2, 3, 4]));
    //     q!(h.now.round_count, 1);
    //     q!(h.now.round_left, 2);
    //     q!(
    //         h.edges,
    //         Edges::from(vec![
    //             vec![],     // 0: root
    //             vec![0],    // 1: mr 1
    //             vec![0, 1], // 2: mr 2
    //             vec![0, 2], // 3: mr 3, mark
    //             vec![1, 3], // 4: sc
    //             vec![],
    //         ])
    //     );
    //     h = h.perform(&Attach(attach_here, 3)).unwrap();
    //     q!(
    //         h.edges,
    //         Edges::from(vec![
    //             vec![],     // 0: root
    //             vec![0],    // 1: mr 1
    //             vec![0, 1], // 2: mr 2
    //             vec![0, 2], // 3: mr 3, mark
    //             vec![1, 3], // 4: sc 1
    //             vec![4],    // 5: ch 1
    //             vec![5],    // 6: ch 2
    //             vec![6],    // 7: ch 3
    //             vec![3, 7], // 8
    //             vec![],
    //         ])
    //     );
    //     {
    //         let part_a = &h.now;

    //         q!(part_a.anchors, Queue::from(vec![4, 5, 6, 7]));
    //         q!(part_a.round_count, 0);
    //         q!(part_a.round_left, 4);
    //     }

    //     h = h.perform(&Sc).unwrap();
    //     q!(h.now.anchors, Queue::from(vec![5, 6, 7, 9]));
    //     q!(h.now.round_count, 1);
    //     q!(h.now.round_left, 3);

    //     q!(
    //         h.edges,
    //         Edges::from(vec![
    //             vec![],     // 0: root
    //             vec![0],    // 1: mr 1
    //             vec![0, 1], // 2: mr 2
    //             vec![0, 2], // 3: mr 3, mark
    //             vec![1, 3], // 4: sc 1
    //             vec![4],    // 5: ch 1
    //             vec![5],    // 6: ch 2
    //             vec![6],    // 7: ch 3
    //             vec![3, 7], // 8: attaching
    //             vec![4, 8], // 9: sc
    //             vec![],
    //         ])
    //     );

    //     h = h.perform(&Sc).unwrap();
    //     q!(h.now.anchors, Queue::from(vec![6, 7, 9, 10]));
    //     h = h.perform(&Sc).unwrap();
    //     q!(h.now.anchors, Queue::from(vec![7, 9, 10, 11]));
    //     h = h.perform(&Sc).unwrap();
    //     h = h.perform(&Sc).unwrap();

    //     let result = h.finish();
    //     q!(result.nodes.len(), result.colors.len());
    // }
}
