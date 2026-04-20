use super::Hook;
use crate::{
    acl::ActionWithOrigin,
    data::{Peculiarity, PointsOnPushPlane},
    graph_construction::{ErrorCode, hook::WorkingLoops},
};
use ErrorCode::*;

pub struct StitchBuilder<'a> {
    hook: Hook,
    anchored: Option<usize>,
    lingering: bool,
    origin: &'a ActionWithOrigin,
}

type Progress<'a> = Result<StitchBuilder<'a>, ErrorCode>;

impl<'a> StitchBuilder<'a> {
    pub fn linger(hook: Hook, origin: &'a ActionWithOrigin) -> Progress<'a> {
        Ok(Self {
            hook,
            anchored: None,
            lingering: true,
            origin,
        })
    }

    pub fn pull_through(mut self) -> Progress<'a> {
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

    fn register_stitch(mut self, accept_single_loop: bool) -> Progress<'a> {
        let peculiarity = if accept_single_loop {
            match self.hook.now.working_on {
                WorkingLoops::Both => None,
                WorkingLoops::Back => Some(Peculiarity::BLO(self.points_on_push_plane()?)),
                WorkingLoops::Front => Some(Peculiarity::FLO(self.points_on_push_plane()?)),
            }
        } else {
            None
        };

        self.hook
            .add_node(self.origin.clone())
            .peculiarity_opt(peculiarity)
            .parent_opt(self.anchored);
        self.hook.now.cursor += 1;
        Ok(self)
    }

    // NOTE why need this accept_single_loop?
    fn pull_over_without_registering_anchor(mut self, accept_single_loop: bool) -> Progress<'a> {
        // NOTE isn't lingering always true?
        if self.lingering {
            let prev = self.hook.previous_stitch();
            self.hook.edges.link(prev, self.hook.now.cursor);
        }

        self.register_stitch(accept_single_loop)
    }

    pub fn pull_over(mut self) -> Progress<'a> {
        self.hook.now.anchors.push_back(self.hook.now.cursor);
        Ok(self.pull_over_without_registering_anchor(true)?)
    }

    pub fn finish(mut self) -> Result<Hook, ErrorCode> {
        if self.anchored.is_some() {
            self = self.next_anchor()
        }
        Ok(self.hook)
    }

    #[allow(dead_code)]
    pub fn chain(mut self, stitches: usize) -> Result<Hook, ErrorCode> {
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
    ) -> Result<(Vec<usize>, Hook), ErrorCode> {
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

    pub fn fasten_off_with_tip(
        mut hook: Hook,
        origin: ActionWithOrigin,
    ) -> Result<Hook, ErrorCode> {
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

        hook.add_node(origin).peculiarity(Peculiarity::Tip);
        hook.now.cursor += 1;
        Ok(hook)
    }

    fn points_on_push_plane(&self) -> Result<PointsOnPushPlane, ErrorCode> {
        let mother = self.anchored.ok_or(SingleLoopOnNonAnchored)?;
        let father = mother + 1;
        let grandparent = self.hook.nodes[mother]
            .parent
            .ok_or(SingleLoopNoGrandparent)?;
        Ok((father, mother, grandparent))
    }
}

#[cfg(test)]
mod tests {
    use super::super::Queue;
    use super::*;
    use crate::{acl::Action::*, data::Edges, graph_construction::hook::HookParams};
    use pretty_assertions::assert_eq as q;

    // TODO
    // test magic ring lower and upper limit
    // test starting with short chain (e.g. Ch(1))
    // test work after FO causes NoAnchorToPullThrough
    // test interaction of single-loop and chains (chains are not anchored)
    // test parents and grandparents around single-loop

    fn mr3() -> Hook {
        let mut h = Hook::new(HookParams::default());
        h = h.perform(&MR(3).without_origin()).unwrap();
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
