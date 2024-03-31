use super::utils::{Peculiarity, WorkingLoops};
use super::Hook;

pub struct Stitch {
    hook: Hook,
}

impl Stitch {
    pub fn linger(mut hook: Hook) -> Self {
        let previous_node = match hook.override_previous_stitch {
            Some(x) => {
                hook.override_previous_stitch = None;
                x
            }
            None => hook.now.cursor - 1,
        };
        hook.edges.link(previous_node, hook.now.cursor);
        Self { hook }
    }

    pub fn pull_through(mut self) -> Self {
        let hook = &mut self.hook;
        hook.edges.link(hook.now.anchor, hook.now.cursor);
        self
    }

    pub fn next_anchor(mut self) -> Self {
        let hook = &mut self.hook;
        hook.now.anchor += 1;
        hook.now.round_left -= 1;
        if hook.now.round_left == 0 {
            hook.round_spans
                .push((hook.now.cursor - hook.now.round_count, hook.now.cursor - 1));
            hook.now.round_left = hook.now.round_count;
            if hook.at_junction {
                hook.now.anchor = hook.now.cursor - hook.now.round_count;
                hook.at_junction = false;
            }
            hook.now.round_count = 0;
        }
        self
    }

    pub fn pull_over(mut self) -> Self {
        self.hook.edges.grow();
        self.hook.colors.push(self.hook.color);
        self.hook.parents.push(Some(self.hook.now.anchor));
        use WorkingLoops::*;
        match self.hook.now.working_on {
            Both => (),
            Back | Front => self.register_single_loop(),
        }
        self.hook.now.cursor += 1;
        self.hook.now.round_count += 1;
        self
    }

    pub fn finish(self) -> Hook {
        self.next_anchor().hook
    }

    fn register_single_loop(&mut self) {
        let hook = &mut self.hook;
        let mother = hook.now.anchor;
        let father = hook.now.anchor + 1;
        let grandparent = hook.parents[hook.now.anchor].expect("Grandparent exists");
        let points_on_push_plane = (father, mother, grandparent);
        let peculiarity = match hook.now.working_on {
            WorkingLoops::Both => unreachable!(),
            WorkingLoops::Back => Peculiarity::BLO(points_on_push_plane),
            WorkingLoops::Front => Peculiarity::FLO(points_on_push_plane),
        };
        hook.peculiar
            .insert(hook.now.cursor, peculiarity)
            .map_or((), |_| panic!("BLO/FLO point is already peculiar"))
    }
}
