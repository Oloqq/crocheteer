use std::collections::HashMap;

use super::{Edges, Hook, Moment, Queue, utils::*};
use crate::{
    ColorRgb,
    acl::{Action::*, Flow},
    hook::hook_result::Peculiarity,
};

const DEFAULT_COLOR: ColorRgb = [255, 0, 255];

impl Hook {
    pub fn from_starting_sequence(flow: &mut impl Flow) -> Result<Self, HookError> {
        let mut action = flow.next().unwrap();
        let mut color = DEFAULT_COLOR;
        if let Color(c) = action {
            color = c;
            action = flow.next().unwrap();
        }
        Self::start_with(&action, color)
    }

    pub fn start_with(action: &Action, color: ColorRgb) -> Result<Self, HookError> {
        match action {
            MR(x) => {
                let edges = {
                    let mut tmp = Edges::new();
                    tmp.grow();
                    tmp
                };
                // TODO replace with from_magic_ring? need to keep the logic separate enough to allow multipart
                let will_be_overwritten_with_magic_ring = Moment {
                    anchors: Queue::new(),
                    cursor: 0,
                    working_on: WorkingLoops::Both,
                    limb_ownerhip: 0,
                };

                let mut result = Self {
                    edges,
                    peculiar: HashMap::new(),
                    now: will_be_overwritten_with_magic_ring,
                    parents: vec![],
                    labels: HashMap::new(),
                    override_previous_stitch: None,
                    color,
                    colors: vec![],
                    last_stitch: None,
                    last_mark: None,
                    mark_to_node: HashMap::new(),
                    tmp_mark_to_node: HashMap::new(),
                    part_limits: vec![],
                    mr_count: 0,
                };
                result.magic_ring(*x);
                Ok(result)
            }
            _ => Err(HookError::BadStarter),
        }
    }
}
