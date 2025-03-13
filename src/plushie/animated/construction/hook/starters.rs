use std::collections::HashMap;

use super::{utils::*, Edges, Hook, Moment, Queue};
use crate::acl::Flow;

const DEFAULT_COLOR: colors::Color = (255, 0, 255);

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

    pub fn start_with(action: &Action, color: colors::Color) -> Result<Self, HookError> {
        match action {
            MRConfigurable(x, label) => {
                let mut hook = Self::start_with(&MR(*x), color)?;
                assert_eq!(hook.peculiar.get(&0), Some(&Peculiarity::Locked));
                hook.mark_to_node.insert(label.clone(), 0);

                Ok(hook)
            }
            MR(x) => {
                let edges = {
                    let mut tmp = Edges::new();
                    tmp.grow();
                    tmp
                };
                let this_will_be_overwritten_how_do_i_design_it_readably_bruh_please_tell_me_via_pr_thanks =
                    Moment {
                        anchors: Queue::new(),
                        cursor: 0,
                        working_on: WorkingLoops::Both,
                        limb_ownerhip: 0,
                    };

                let mut result = Self {
                    edges,
                    peculiar: HashMap::new(),
                    now: this_will_be_overwritten_how_do_i_design_it_readably_bruh_please_tell_me_via_pr_thanks,
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
