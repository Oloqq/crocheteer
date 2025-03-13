use HookError::*;

use super::{
    utils::{HookError, Peculiarity, WorkingLoops},
    Hook,
};

pub struct Stitch {
    hook: Hook,
    anchored: Option<usize>,
    lingering: bool,
}

type Progress = Result<Stitch, HookError>;

impl Stitch {
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

    pub fn next_anchor(mut self) -> Progress {
        let hook = &mut self.hook;
        hook.now.anchors.pop_front().expect("there was an anchor");
        Ok(self)
    }

    fn register_stitch(mut self) -> Progress {
        self.hook.edges.grow();
        self.hook.colors.push(self.hook.color);
        self.hook.parents.push(self.anchored);
        self.hook.now.cursor += 1;
        Ok(self)
    }

    fn pull_over_without_registering_anchor(mut self, accept_single_loop: bool) -> Progress {
        if self.lingering {
            let prev = self.hook.previous_stitch();
            self.hook.edges.link(prev, self.hook.now.cursor);
        }

        if accept_single_loop {
            use WorkingLoops::*;
            match self.hook.now.working_on {
                Both => (),
                Back | Front => self.register_single_loop()?,
            }
        }

        Ok(self.register_stitch()?)
    }

    pub fn pull_over(mut self) -> Progress {
        self.hook.now.anchors.push_back(self.hook.now.cursor);
        Ok(self.pull_over_without_registering_anchor(true)?)
    }

    pub fn finish(mut self) -> Result<Hook, HookError> {
        if self.anchored.is_some() {
            self = self.next_anchor()?
        }
        Ok(self.hook)
    }

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
        hook.peculiar.insert(tip, Peculiarity::Tip);
        hook.colors.push(hook.color);
        hook.now.cursor += 1;
        Ok(hook)
    }

    fn register_single_loop(&mut self) -> Result<(), HookError> {
        let hook = &mut self.hook;
        let mother = self.anchored.ok_or(SingleLoopOnNonAnchored)?;
        let father = mother + 1;
        let grandparent = hook.parents[mother].ok_or(SingleLoopNoGrandparent)?;
        let points_on_push_plane = (father, mother, grandparent);
        let peculiarity = match hook.now.working_on {
            WorkingLoops::Both => unreachable!(),
            WorkingLoops::Back => Peculiarity::BLO(points_on_push_plane),
            WorkingLoops::Front => Peculiarity::FLO(points_on_push_plane),
        };
        let _ = hook
            .peculiar
            .insert(hook.now.cursor, peculiarity.clone())
            .map_or((), |prev| {
                panic!("BLO/FLO point is already peculiar. was: {prev:?} new: {peculiarity:?}")
            });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq as q;

    use super::{
        super::{super::hook_result::Edges, utils::*},
        *,
    };

    // test magic ring lower and upper limit
    // test starting with short chain (e.g. Ch(1))
    // test work after FO causes NoAnchorToPullThrough
    // test interaction of single-loop and chains (chains are not anchored)
    // test parents and grandparents around single-loop

    const COLOR: colors::Color = colors::RED;

    fn mr3() -> Hook {
        let h = Hook::start_with(&MR(3), COLOR).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        q!(h.now.cursor, 4);
        q!(
            h.edges,
            Edges::from_unchecked(vec![vec![], vec![0], vec![0, 1], vec![0, 2], vec![]])
        );
        h
    }

    #[test]
    fn test_goto_without_fo() {
        let mut h = mr3();
        h = h.test_perform(&Mark(0)).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        h = h.test_perform(&Sc).unwrap();
        h = h.test_perform(&Sc).unwrap();
        h = h.test_perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from([4, 5, 6]));
        q!(h.now.cursor, 7);
        h = h.test_perform(&Goto(0)).unwrap();
        q!(h.now.anchors, Queue::from([1, 2, 3]));
        q!(h.now.cursor, 7);
        h = h.test_perform(&Sc).unwrap();
        h = h.test_perform(&Sc).unwrap();
        h = h.test_perform(&Sc).unwrap();
        q!(h.now.anchors, Queue::from([7, 8, 9]));
    }
}
