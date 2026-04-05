use HookError::*;

use crate::hook::{
    WorkingLoops,
    node::{Peculiarity, PointsOnPushPlane},
};

use super::{Hook, errors::HookError};

pub struct StitchBuilder {
    hook: Hook,
    anchored: Option<usize>,
    lingering: bool,
}

type Progress = Result<StitchBuilder, HookError>;

impl StitchBuilder {
    pub fn linger(hook: Hook) -> Progress {
        Ok(Self {
            hook,
            anchored: None,
            lingering: true,
        })
    }

    pub fn pull_through(mut self) -> Progress {
        let hook = &mut self.hook;
        let anchor = *hook.now.anchors.front().ok_or(NoAnchorToPullThrough)?;
        hook.edges.link(anchor, hook.now.cursor);
        self.anchored = Some(anchor);
        Ok(self)
    }

    pub fn next_anchor(mut self) -> Self {
        let hook = &mut self.hook;
        hook.now.anchors.pop_front().expect("this should only be reachable after a successful pull_through, which is responsible for reporting an Err");
        self
    }

    fn register_stitch(mut self, accept_single_loop: bool) -> Progress {
        let peculiarity = if accept_single_loop {
            match self.hook.now.working_on {
                WorkingLoops::Both => None,
                WorkingLoops::Back => Some(Peculiarity::BLO(self.points_on_push_plane()?)),
                WorkingLoops::Front => Some(Peculiarity::FLO(self.points_on_push_plane()?)),
            }
        } else {
            None
        };

        self.hook.edges.grow();
        self.hook.add_node(peculiarity);
        self.hook.parents.push(self.anchored);
        self.hook.now.cursor += 1;
        Ok(self)
    }

    // NOTE why need this accept_single_loop?
    fn pull_over_without_registering_anchor(mut self, accept_single_loop: bool) -> Progress {
        // NOTE isn't lingering always true?
        if self.lingering {
            let prev = self.hook.previous_stitch();
            self.hook.edges.link(prev, self.hook.now.cursor);
        }

        self.register_stitch(accept_single_loop)
    }

    pub fn pull_over(mut self) -> Progress {
        self.hook.now.anchors.push_back(self.hook.now.cursor);
        Ok(self.pull_over_without_registering_anchor(true)?)
    }

    pub fn finish(mut self) -> Result<Hook, HookError> {
        if self.anchored.is_some() {
            self = self.next_anchor()
        }
        Ok(self.hook)
    }

    #[allow(dead_code)]
    pub fn chain(mut self, stitches: usize) -> Result<Hook, HookError> {
        if stitches == 0 {
            return Err(ChainOfZero);
        }

        self.hook.now.anchors.push_front(self.hook.now.cursor);
        self = self.pull_over_without_registering_anchor(true)?;

        // skip first and last
        for _ in 2..stitches {
            self.hook.now.anchors.push_front(self.hook.now.cursor);
            self = self.pull_over_without_registering_anchor(false)?;
        }
        self = self.pull_over_without_registering_anchor(false)?;
        self.finish()
    }

    pub fn attaching_chain(
        mut self,
        stitches: usize,
        attach_to: usize,
    ) -> Result<(Vec<usize>, Hook), HookError> {
        if stitches == 0 {
            return Err(ChainOfZero); // TODO
        }

        let mut new_anchors = Vec::with_capacity(stitches);

        new_anchors.push(self.hook.now.cursor);
        self = self.pull_over_without_registering_anchor(true)?;

        // skip first
        for _ in 1..stitches {
            new_anchors.push(self.hook.now.cursor);
            self = self.pull_over_without_registering_anchor(false)?;
        }

        new_anchors.push(self.hook.now.cursor);
        self.hook.edges.link(self.hook.now.cursor, attach_to);
        self = self.pull_over_without_registering_anchor(false)?;

        Ok((new_anchors, self.hook))
    }

    pub fn fasten_off_with_tip(mut hook: Hook) -> Result<Hook, HookError> {
        if hook.now.anchors.len() < 2 {
            log::debug!("No anchors to fasten off");
            return Err(FORequires2Anchors);
        }

        let tip = hook.now.cursor;
        let anchors_num = hook.now.anchors.len();
        const ANCHORS_FOR_FO_LIMIT: usize = 12;
        if anchors_num > ANCHORS_FOR_FO_LIMIT {
            log::debug!(
                "Too many anchors for FO (limit: {ANCHORS_FOR_FO_LIMIT}, got: {anchors_num})"
            );
            return Err(TooManyAnchorsForFO);
        }

        while let Some(anchor) = hook.now.anchors.pop_front() {
            hook.edges.link(anchor, tip);
        }

        hook.edges.grow();
        hook.add_node(Some(Peculiarity::Tip));
        hook.now.cursor += 1;
        Ok(hook)
    }

    fn points_on_push_plane(&self) -> Result<PointsOnPushPlane, HookError> {
        let mother = self.anchored.ok_or(SingleLoopOnNonAnchored)?;
        let father = mother + 1;
        let grandparent = self.hook.parents[mother].ok_or(SingleLoopNoGrandparent)?;
        Ok((father, mother, grandparent))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq as q;

    use crate::{
        ColorRgb,
        acl::Action::*,
        hook::{HookParams, edges::Edges},
    };

    use super::{super::errors::*, *};
    const COLOR: ColorRgb = [255, 0, 0];

    // TODO
    // test magic ring lower and upper limit
    // test starting with short chain (e.g. Ch(1))
    // test work after FO causes NoAnchorToPullThrough
    // test interaction of single-loop and chains (chains are not anchored)
    // test parents and grandparents around single-loop

    fn mr3() -> Hook {
        let h = Hook::start_with(&MR(3), COLOR, HookParams::default()).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        q!(h.now.cursor, 4);
        q!(
            h.edges,
            Edges::from(vec![vec![], vec![0], vec![0, 1], vec![0, 2], vec![]])
        );
        h
    }

    #[test]
    fn test_goto_without_fo() {
        let mut h = mr3();
        h = h.test_perform(&Mark("0".into())).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        h = h.test_perform(&Sc).unwrap();
        h = h.test_perform(&Sc).unwrap();
        h = h.test_perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from([4, 5, 6]));
        q!(h.now.cursor, 7);
        h = h.test_perform(&Goto("0".into())).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        q!(h.now.cursor, 7);
        h = h.test_perform(&Sc).unwrap();
        h = h.test_perform(&Sc).unwrap();
        h = h.test_perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from([7, 8, 9]));
    }
}
