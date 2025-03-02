pub mod leniency;
mod starters;
mod state_mgmt;
#[cfg(test)]
mod tests;
mod utils;
mod working_stitch;

use std::collections::HashMap;

use leniency::Leniency;

use self::{utils::*, working_stitch::Stitch, HookError::*};
use super::hook_result::{Edges, InitialGraph};
use crate::{
    acl::{
        actions::{Action, Label},
        Flow,
    },
    plushie::params::HookParams,
};

#[derive(Clone, Debug)]
struct Moment {
    /// Node index to be created
    cursor: usize,
    anchors: Queue<usize>,
    working_on: WorkingLoops,
    /// Moments on unconnected graphs will have different number
    limb_ownerhip: usize,
}

/// Responsible for building the graph used in the simulation
#[derive(Clone, Debug)]
pub struct Hook {
    edges: Edges,
    peculiar: HashMap<usize, Peculiarity>,
    now: Moment,
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
    /// Map from labels the index of the node they affect. For now works with just MRConfigurable.
    mark_to_node: HashMap<String, usize>,
    /// Indexes where parts begin and end. At the end, first element should be zero, last element should be colors.len()
    part_limits: Vec<usize>,
    /// Used to track unconnected limbs
    mr_count: usize,
}

fn split_moment(
    source: &mut Moment,
    attachment_anchor: usize,
    new_anchors: Vec<usize>,
) -> (Moment, Moment) {
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
        cursor: source.cursor,
        anchors: ring_a,
        working_on: WorkingLoops::Both,
        limb_ownerhip: source.limb_ownerhip,
    };

    let moment_b = Moment {
        cursor: source.cursor,
        anchors: ring_b.clone(),
        working_on: WorkingLoops::Both,
        limb_ownerhip: source.limb_ownerhip,
    };

    (moment_a, moment_b)
}

impl Hook {
    pub fn parse(mut flow: impl Flow, params: &HookParams) -> Result<InitialGraph, HookError> {
        if flow.peek().is_none() {
            return Err(Empty);
        }
        let mut hook = Hook::from_starting_sequence(&mut flow)?;
        let mut i: u32 = 0;
        while let Some(action) = flow.next() {
            log::trace!("Performing [{i}] {action:?}");
            i += 1;
            hook = hook.perform(&action, params)?;
        }

        let result = hook.finish();
        Ok(result)
    }

    fn finish(mut self) -> InitialGraph {
        self.edges.cleanup();
        self.part_limits.push(self.now.cursor);
        InitialGraph {
            edges: self.edges,
            peculiarities: self.peculiar,
            colors: self.colors,
            mark_to_node: self.mark_to_node,
            part_limits: self.part_limits,
        }
    }

    fn do_perform(mut self, action: &Action, params: &HookParams) -> Result<Self, HookError> {
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
            Slst => {
                let anchor = self.now.anchors.pop_front().ok_or(NoAnchorToPullThrough)?;
                self.edges.link(self.now.cursor - 1, anchor);
                self.override_previous_stitch = Some(anchor);
                self.now.anchors.push_back(anchor);
            }
            Ch(x) => {
                if matches!(self.last_stitch, Some(Ch(_))) {
                    return Err(ChainAfterChain);
                }
                self = Stitch::linger(self)?.chain(*x)?;
            }
            Attach(label, chain_size) => {
                log::debug!("attach to label: {label}");
                // FIXME for now, assuming that chain_size > 0 connects to the same limb
                // and chain_size = 0 connects to another limb
                // TEMP attach_merge
                if *chain_size == 997 {
                    self = self.attach_merge(label)?;
                } else if *chain_size > 0 {
                    self = self.attach_with_chain(label, chain_size)?;
                } else {
                    self = self.attach_directly(label)?;
                }
            }
            Reverse => unimplemented!(),
            FLO => self.now.working_on = WorkingLoops::Front,
            BLO => self.now.working_on = WorkingLoops::Back,
            BL => self.now.working_on = WorkingLoops::Both,
            Goto(label) => self.restore(*label)?,
            Mark(label) => self.save(*label)?,
            MR(_) => return Err(AnonymousMrInTheMiddle),
            MRConfigurable(x, label) => {
                self.override_previous_stitch = None;
                self.mark_to_node.insert(label.clone(), self.now.cursor);
                self.magic_ring(*x);
            }
            FO => {
                if params.tip_from_fo {
                    self = Stitch::fasten_off_with_tip(self)?
                }
            }
            Color(c) => self.color = *c,
            EnforceAnchors(expected, location) => {
                let actual = self.now.anchors.len();
                if params.enforce_counts && actual != *expected {
                    return Err(HookError::WrongAnnotation {
                        expected: *expected,
                        actual,
                        location: *location,
                    });
                }
            }
        };

        match action {
            MR(..) => unreachable!("MR allowed inside the pattern is stored as MRConfigurable"),
            Reverse | FLO | BLO | BL | Goto(_) | FO | Action::Color(_) => self.last_mark = None,
            Mark(_) => self.last_mark = Some(action.clone()),
            _ => {
                self.last_stitch = Some(action.clone());
                self.last_mark = None
            }
        }

        Ok(self)
    }

    pub fn perform(self, action: &Action, params: &HookParams) -> Result<Self, HookError> {
        match self.leniency {
            Leniency::NoMercy => self.do_perform(action, params),
            Leniency::SkipIncorrect => {
                // If this approach turns out to be actually useful, a more efficient implementation is necessary
                let copy = self.clone();
                match copy.do_perform(action, params) {
                    Ok(hook) => Ok(hook),
                    Err(_) => Ok(self),
                }
            }
            Leniency::GeneticFixups => todo!(),
        }
    }

    fn previous_stitch(&mut self) -> usize {
        match self.override_previous_stitch {
            Some(x) => {
                self.override_previous_stitch = None;
                x
            }
            None => self.now.cursor - 1,
        }
    }

    fn attach_with_chain(mut self, label: &usize, chain_size: &usize) -> Result<Self, HookError> {
        // FIXME this should probably affect part_limits
        // FIXME part_limits should prolly be limb_limits
        // create a chain
        // attach it to given point
        // assumption: user marked a spot X right before an attach
        // after attaching, the plushie splits into 2 working rings: A and B
        // A is the one that user will work when doing rounds normally
        // ring B can be accessed by goto(X)

        let starting_anchor = self.now.cursor;
        // TODO won't this panic?
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
        Ok(self)
    }

    fn attach_directly(mut self, label: &usize) -> Result<Self, HookError> {
        let cursor_at = self.now.cursor;
        let target = self.labels.get(&label).ok_or(UnknownLabel(*label))?;
        if self.now.limb_ownerhip != target.limb_ownerhip {
            // this action connects previously unconnected graphs
            self.part_limits.push(cursor_at);
            self.merge_limb_ownership(self.now.limb_ownerhip, target.limb_ownerhip);
        }

        let x = self.previous_stitch();
        self.restore(*label)?;
        self.override_previous_stitch = Some(x);

        Ok(self)
    }

    fn attach_merge(mut self, label: &usize) -> Result<Self, HookError> {
        let mut target = self.labels.get(&label).ok_or(UnknownLabel(*label))?.clone();
        assert!(self.now.limb_ownerhip == target.limb_ownerhip);

        self.override_previous_stitch = Some(self.previous_stitch());
        target.cursor = self.now.cursor;
        target.anchors.append(&mut self.now.anchors);
        // target.round_left += self.now.round_left;
        self.now = target;

        Ok(self)
    }

    fn merge_limb_ownership(&mut self, main: usize, appendix: usize) {
        for (_, moment) in &mut self.labels {
            if moment.limb_ownerhip == appendix {
                moment.limb_ownerhip = main;
            }
        }
    }

    fn split_moment(mut self, attachment_anchor: usize, new_anchors: Vec<usize>) -> (Self, Moment) {
        let (moment_a, moment_b) = split_moment(&mut self.now, attachment_anchor, new_anchors);
        self.now = moment_a;
        (self, moment_b)
    }

    fn magic_ring(&mut self, size: usize) {
        assert_eq!(self.edges.last().unwrap().len(), 0);

        self.part_limits.push(self.now.cursor);
        let ring_root = self.now.cursor;
        let ring_end = ring_root + size;

        // spot for ring root in edges is already created
        self.parents.push(None); // ring root has no parent
        self.colors.push(self.color);
        for _ in 0..size {
            self.edges.grow();
            self.parents.push(Some(ring_root));
            self.colors.push(self.color);
        }
        self.edges.grow(); // prepare place for the next node

        // connect outer nodes to ring root
        for connected_to_root in ring_root + 1..=ring_end {
            self.edges.link(ring_root, connected_to_root);
        }
        // connect outer nodes to each other
        for outer_ring_stitch in ring_root + 1..ring_end {
            self.edges.link(outer_ring_stitch, outer_ring_stitch + 1);
        }

        self.peculiar.insert(ring_root, Peculiarity::Locked);

        self.now = Moment {
            anchors: Queue::from_iter(ring_root + 1..=ring_end),
            cursor: ring_end + 1,
            working_on: WorkingLoops::Both,
            limb_ownerhip: self.mr_count,
        };
        self.mr_count += 1;

        assert_eq!(self.edges.last().unwrap().len(), 0);
    }
}
