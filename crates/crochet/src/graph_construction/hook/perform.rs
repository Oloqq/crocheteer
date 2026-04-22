use crate::{
    acl::{Action, ActionWithOrigin},
    graph_construction::{
        ErrorCode,
        hook::{DeferredEdge, Moment},
    },
};

use super::{Hook, StitchBuilder, WorkingLoops};

impl Hook {
    pub(crate) fn perform(
        mut self,
        action_with_origin: &ActionWithOrigin,
    ) -> Result<Self, ErrorCode> {
        use Action::*;
        use ErrorCode::*;

        let part_start = *self.part_limits.last().unwrap_or(&0);
        if self.now.cursor == part_start {
            match &action_with_origin.action {
                BeginPart | EndPart | MR(..) | Color(..) => (),
                _ => return Err(ErrorCode::BadStarter),
            }
        }

        match &action_with_origin.action {
            Sc => {
                self = StitchBuilder::linger(self, action_with_origin)?
                    .pull_through()?
                    .pull_over()?
                    .finish()?
            }
            Inc => {
                self = StitchBuilder::linger(self, action_with_origin)?
                    .pull_through()?
                    .pull_over()?
                    .pull_through()?
                    .pull_over()?
                    .finish()?;
            }
            Dec => {
                self = StitchBuilder::linger(self, action_with_origin)?
                    .pull_through()?
                    .next_anchor()
                    .pull_through()?
                    .pull_over()?
                    .finish()?;
            }
            Slst => {
                log::error!("slst is disabled");
                // let anchor = self.now.anchors.pop_front().ok_or(NoAnchorToPullThrough)?;
                // self.edges.link(self.now.cursor - 1, anchor);
                // self.override_previous_node = Some(anchor);
                // self.now.anchors.push_back(anchor);
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
                    self = self.attach_with_chain(label, chain_size, action_with_origin)?;
                } else {
                    self = self.attach_directly(label)?;
                }
            }
            FLO => self.now.working_on = WorkingLoops::Front,
            BLO => self.now.working_on = WorkingLoops::Back,
            BL => self.now.working_on = WorkingLoops::Both,
            Goto(label) => self.restore(label)?,
            Mark(label) => self.save(label)?,
            MR(count) => {
                self.magic_ring(*count, action_with_origin);
            }
            BeginPart => {}
            EndPart => {
                self.part_limits.push(self.now.cursor);
                self.part_cursor += 1;
                self.now = Moment {
                    cursor: self.now.cursor,
                    part: self.part_cursor,
                    ..Default::default()
                };
            }
            FO => {
                if self.params.tip_from_fo {
                    self = StitchBuilder::fasten_off_with_tip(self, action_with_origin.clone())?
                }
            }
            Color(c) => self.color = *c,
            EnforceAnchors(expected, location) => {
                let actual = self.now.anchors.len();
                if self.params.enforce_counts && actual != *expected {
                    return Err(ErrorCode::WrongAnnotation {
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
                let lpart = part_of_node(&self.part_limits, left);
                let rpart = part_of_node(&self.part_limits, right);
                let happens_with_node = self.nodes.len();
                if lpart != rpart {
                    self.part_joins
                        .register_part_join(lpart, rpart, happens_with_node);
                }

                self.deferred_edges.push(DeferredEdge {
                    with_node: happens_with_node,
                    node_a: *left,
                    node_b: *right,
                });
            }
        };

        match &action_with_origin.action {
            FLO
            | BLO
            | BL
            | Goto(_)
            | FO
            | Action::Color(_)
            | Sew(..)
            | EnforceAnchors(..)
            | BeginPart
            | EndPart => self.last_mark = None,
            Mark(_) => self.last_mark = Some(action_with_origin.action.clone()),
            Sc | Dec | Inc | Slst | Attach(..) | MR(_) => {
                self.last_stitch = Some(action_with_origin.action.clone());
                self.last_mark = None
            }
        }

        Ok(self)
    }
}

fn part_of_node(part_limits: &Vec<usize>, node: &usize) -> usize {
    for (i, end) in part_limits.iter().enumerate() {
        if node < end {
            return i;
        }
    }
    return part_limits.len();
}
