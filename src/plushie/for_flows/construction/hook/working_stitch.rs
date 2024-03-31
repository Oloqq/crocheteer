use super::utils::{HookError, Peculiarity, WorkingLoops};
use super::Hook;

use HookError::*;

pub struct Stitch {
    hook: Hook,
    anchored: Option<usize>,
}

type Progress = Result<Stitch, HookError>;

impl Stitch {
    pub fn linger(mut hook: Hook) -> Progress {
        let prev = previous_stitch(&mut hook);
        hook.edges.link(prev, hook.now.cursor);
        Ok(Self {
            hook,
            anchored: None,
        })
    }

    pub fn pull_through(mut self) -> Progress {
        let hook = &mut self.hook;
        let anchor = *hook.now.anchors.front().ok_or(NoAnchorToPullThrough)?;
        hook.edges.link(anchor, hook.now.cursor);
        self.anchored = Some(anchor);
        use WorkingLoops::*;
        match self.hook.now.working_on {
            Both => (),
            Back | Front => self.register_single_loop(),
        }
        Ok(self)
    }

    pub fn next_anchor(mut self) -> Progress {
        let hook = &mut self.hook;
        hook.now.anchors.pop_front().expect("there was an anchor");
        hook.now.round_left -= 1;
        if hook.now.round_left == 0 {
            hook.round_spans
                .push((hook.now.cursor - hook.now.round_count, hook.now.cursor - 1));
            hook.now.round_left = hook.now.round_count;
            if hook.at_junction {
                todo!()
                // hook.now.anchor = hook.now.cursor - hook.now.round_count;
                // hook.at_junction = false;
            }
            hook.now.round_count = 0;
        }
        Ok(self)
    }

    pub fn pull_over(mut self) -> Progress {
        self.hook.edges.grow();
        self.hook.colors.push(self.hook.color);
        self.hook.parents.push(self.anchored);
        self.hook.now.anchors.push_back(self.hook.now.cursor);
        self.hook.now.cursor += 1;
        self.hook.now.round_count += 1;
        Ok(self)
    }

    pub fn finish(self) -> Result<Hook, HookError> {
        Ok(self.next_anchor()?.hook)
    }

    pub fn fasten_off_with_tip(mut hook: Hook) -> Result<Hook, HookError> {
        if hook.now.anchors.len() < 2 {
            return Err(FORequires2Anchors);
        }

        let tip = hook.now.cursor;
        while let Some(anchor) = hook.now.anchors.pop_front() {
            hook.edges.link(anchor, tip);
        }

        hook.edges.grow();
        hook.peculiar.insert(tip, Peculiarity::Tip);
        hook.round_spans.push((tip, tip));
        hook.parts.push((hook.part_start, tip));
        hook.colors.push(hook.color);
        hook.now.cursor += 1;
        Ok(hook)
    }

    fn register_single_loop(&mut self) {
        todo!()
        // let hook = &mut self.hook;
        // let mother = hook.now.anchors.front().expect("There is an anchor")
        // let father = hook.now.anchor + 1;
        // let grandparent = hook.parents[hook.now.anchor].expect("Grandparent exists");
        // let points_on_push_plane = (father, mother, grandparent);
        // let peculiarity = match hook.now.working_on {
        //     WorkingLoops::Both => unreachable!(),
        //     WorkingLoops::Back => Peculiarity::BLO(points_on_push_plane),
        //     WorkingLoops::Front => Peculiarity::FLO(points_on_push_plane),
        // };
        // hook.peculiar
        //     .insert(hook.now.cursor, peculiarity)
        //     .map_or((), |_| panic!("BLO/FLO point is already peculiar"))
    }
}

fn previous_stitch(hook: &mut Hook) -> usize {
    match hook.override_previous_stitch {
        Some(x) => {
            hook.override_previous_stitch = None;
            x
        }
        None => hook.now.cursor - 1,
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use pretty_assertions::assert_eq as q;

    // test magic ring lower and upper limit
    // test starting with short chain (e.g. Ch(1))
}
