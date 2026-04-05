mod edges;
mod errors;
pub mod hook_result;
mod mark_and_goto;
mod starters;
mod stitch_builder;

use self::{errors::*, stitch_builder::StitchBuilder};
use crate::{
    ColorRgb,
    acl::{
        Action::{self, *},
        Flow, Label,
    },
    hook::{edges::Edges, hook_result::Peculiarity},
};
use HookError::*;
pub use errors::HookError;
use hook_result::InitialGraph;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct HookParams {
    pub tip_from_fo: bool,
    pub enforce_counts: bool,
}

#[derive(Clone, Debug)]
pub enum WorkingLoops {
    Both,
    Back,
    Front,
}

#[derive(Clone, Debug)]
struct Moment {
    /// Node index to be created
    cursor: usize,
    anchors: Queue<usize>,
    working_on: WorkingLoops,
    /// Moments on unconnected graphs shall have different number
    limb_ownerhip: usize,
}

/// Responsible for building the graph used in the simulation
#[derive(Clone, Debug)]
pub struct Hook {
    params: HookParams,
    edges: Edges,
    peculiar: HashMap<usize, Peculiarity>,
    now: Moment,
    /// Storage of index -> it's anchor, used for single loop forces
    parents: Vec<Option<usize>>,
    /// Storage of spots for Mark and Goto
    labels: HashMap<Label, Moment>,
    /// Current color/yarn. Not stored in Moment as typically yarn changes happpen independently of switching positions.
    color: ColorRgb,
    /// Storage of index -> it's color.
    colors: Vec<ColorRgb>,
    // Previous stitch might need to be overwritten after a Goto
    override_previous_stitch: Option<usize>,
    /// Last stitch created (not counting actions like mark, goto)
    last_stitch: Option<Action>,
    /// Was the last action a mark?
    last_mark: Option<Action>,
    /// Map from labels to the index of the node they are on.
    mark_to_node: HashMap<Label, usize>,
    /// Indexes where parts begin and end. When Hook finishes, first element should be equal to zero, last element should be equal to colors.len()
    part_limits: Vec<usize>,
    /// Used to track unconnected limbs
    mr_count: usize,
}

impl Hook {
    pub fn parse(mut flow: impl Flow, params: HookParams) -> Result<InitialGraph, HookError> {
        if flow.peek().is_none() {
            return Err(Empty);
        }
        let mut hook = Hook::from_starting_sequence(&mut flow, params)?;
        let mut i: u32 = 0;
        while let Some(action) = flow.next() {
            log::trace!("Performing [{i}] {action:?}");
            i += 1;
            hook = hook.perform(&action)?;
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

    pub fn perform(mut self, action: &Action) -> Result<Self, HookError> {
        match action {
            Sc => {
                self = StitchBuilder::linger(self)?
                    .pull_through()?
                    .pull_over()?
                    .finish()?
            }
            Inc => {
                self = StitchBuilder::linger(self)?
                    .pull_through()?
                    .pull_over()?
                    .pull_through()?
                    .pull_over()?
                    .finish()?;
            }
            Dec => {
                self = StitchBuilder::linger(self)?
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
            Attach(label, chain_size) => {
                log::debug!("attach to label: {label}");
                // FIXME for now, assuming that chain_size > 0 connects to the same limb
                // and chain_size = 0 connects to another limb
                // TODO how to do the following nicely?
                // see heart pattern for reference (requires multipart)
                // attach_directly corresponds to the first stitch that connects 2 parts
                // it moves to the Moment of the part it is connecting to
                // attach_merge_anchors corresponds to the second stitch that connects 2 parts
                // it creates a single working round from the rounds on 2 parts
                if *chain_size == 997 {
                    self = self.attach_merge_anchors(label)?;
                } else if *chain_size > 0 {
                    self = self.attach_with_chain(label, chain_size)?;
                } else {
                    self = self.attach_directly(label)?;
                }
            }
            FLO => self.now.working_on = WorkingLoops::Front,
            BLO => self.now.working_on = WorkingLoops::Back,
            BL => self.now.working_on = WorkingLoops::Both,
            Goto(label) => self.restore(label)?,
            Mark(label) => self.save(label)?,
            MR(_) => return Err(AnonymousMrInTheMiddle),
            FO => {
                if self.params.tip_from_fo {
                    self = StitchBuilder::fasten_off_with_tip(self)?
                }
            }
            Color(c) => self.color = *c,
            EnforceAnchors(expected, location) => {
                let actual = self.now.anchors.len();
                if self.params.enforce_counts && actual != *expected {
                    return Err(HookError::WrongAnnotation {
                        expected: *expected,
                        actual,
                        location: *location,
                    });
                }
            }
            Sew(left, right) => {
                let Some(left) = self.mark_to_node.get(left) else {
                    return Err(UnknownLabel(left.clone()));
                };
                let Some(right) = self.mark_to_node.get(right) else {
                    return Err(UnknownLabel(right.clone()));
                };

                self.edges.link(*left, *right);
            }
        };

        match action {
            MR(..) => unreachable!("MR allowed inside the pattern is stored as MRConfigurable"),
            FLO | BLO | BL | Goto(_) | FO | Action::Color(_) | Sew(..) => self.last_mark = None,
            Mark(_) => self.last_mark = Some(action.clone()),
            _ => {
                self.last_stitch = Some(action.clone());
                self.last_mark = None
            }
        }

        Ok(self)
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

    fn attach_with_chain(mut self, label: &Label, chain_size: &usize) -> Result<Self, HookError> {
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
            StitchBuilder::linger(self)?.attaching_chain(*chain_size, attachment_anchor)?;
        let mut moment_b;
        (self, moment_b) = self.split_moment(attachment_anchor, new_anchors);
        // let ring_b = self.split_current_moment(attaching_anchor, new_anchors);

        if let Some(Mark(ring_b_label)) = &self.last_mark {
            assert!(self.labels.contains_key(ring_b_label));
            moment_b.cursor = starting_anchor;
            self.labels.insert(ring_b_label.clone(), moment_b);
        }
        Ok(self)
    }

    fn attach_directly(mut self, label: &Label) -> Result<Self, HookError> {
        let cursor_at = self.now.cursor;
        let target = self
            .labels
            .get(label)
            .ok_or_else(|| UnknownLabel(label.clone()))?;
        if self.now.limb_ownerhip != target.limb_ownerhip {
            // this action connects previously unconnected graphs
            self.part_limits.push(cursor_at);
            self.merge_limb_ownership(self.now.limb_ownerhip, target.limb_ownerhip);
        }

        let x = self.previous_stitch();
        self.restore(label)?;
        self.override_previous_stitch = Some(x);

        Ok(self)
    }

    fn attach_merge_anchors(mut self, label: &Label) -> Result<Self, HookError> {
        let mut target = self
            .labels
            .get(label)
            .ok_or_else(|| UnknownLabel(label.clone()))?
            .clone();
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

#[cfg(test)]
mod tests;
