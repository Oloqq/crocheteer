use crate::{
    acl::{Action, ActionWithOrigin, Label},
    graph_construction::hook::{
        ErrorCode, Hook, Moment, WorkingLoops, stitch_builder::StitchBuilder,
    },
};

impl Hook {
    pub(super) fn attach_with_chain(
        mut self,
        label: &Label,
        chain_size: &usize,
        origin: &ActionWithOrigin,
    ) -> Result<Self, ErrorCode> {
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
            StitchBuilder::linger(self, origin)?.attaching_chain(*chain_size, attachment_anchor)?;
        let mut moment_b;
        (self, moment_b) = self.split_moment(attachment_anchor, new_anchors);
        // let ring_b = self.split_current_moment(attaching_anchor, new_anchors);

        if let Some(Action::Mark(ring_b_label)) = &self.last_mark {
            assert!(self.labels.contains_key(ring_b_label));
            moment_b.cursor = starting_anchor;
            self.labels.insert(ring_b_label.clone(), moment_b);
        }
        Ok(self)
    }

    pub(super) fn attach_directly(mut self, label: &Label) -> Result<Self, ErrorCode> {
        let cursor_at = self.now.cursor;
        let target = self
            .labels
            .get(label)
            .ok_or_else(|| ErrorCode::UnknownLabel(label.clone()))?;
        if self.now.part != target.part {
            // this action connects previously unconnected graphs
            self.part_limits.push(cursor_at);
            self.merge_limb_ownership(self.now.part, target.part);
        }

        let x = self.previous_stitch();
        self.restore(label)?;
        self.override_previous_node = Some(x);

        Ok(self)
    }

    pub(super) fn attach_merge_anchors(mut self, label: &Label) -> Result<Self, ErrorCode> {
        let mut target = self
            .labels
            .get(label)
            .ok_or_else(|| ErrorCode::UnknownLabel(label.clone()))?
            .clone();
        assert!(self.now.part == target.part);

        self.override_previous_node = Some(self.previous_stitch());
        target.cursor = self.now.cursor;
        target.anchors.append(&mut self.now.anchors);
        // target.round_left += self.now.round_left;
        self.now = target;

        Ok(self)
    }

    fn merge_limb_ownership(&mut self, main: usize, appendix: usize) {
        for (_, moment) in &mut self.labels {
            if moment.part == appendix {
                moment.part = main;
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
        part: source.part,
    };

    let moment_b = Moment {
        cursor: source.cursor,
        anchors: ring_b.clone(),
        working_on: WorkingLoops::Both,
        part: source.part,
    };

    (moment_a, moment_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_moment() {
        let mut source = Moment {
            cursor: 20,
            anchors: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].into(),
            working_on: WorkingLoops::Both,
            part: 0,
        };
        let (moment_a, moment_b) = split_moment(&mut source, 6, [13, 14, 15, 16].into());
        println!("{:?} {:?}", moment_a.anchors, moment_b.anchors);
        assert_eq!(moment_a.anchors.len(), 9);
        assert_eq!(moment_b.anchors.len(), 9);
    }
}
