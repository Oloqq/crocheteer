use std::collections::HashMap;

use super::{Edges, Hook, Moment, Queue, errors::*};
use crate::{
    ColorRgb,
    acl::{Action::*, Flow},
    hook::{HookParams, WorkingLoops, node::Peculiarity},
};

const DEFAULT_COLOR: ColorRgb = [255, 0, 255];

impl Hook {
    pub fn from_starting_sequence(
        flow: &mut impl Flow,
        params: HookParams,
    ) -> Result<Self, HookError> {
        let mut action = flow.next().unwrap();
        let mut color = DEFAULT_COLOR;
        if let Color(c) = action {
            color = c;
            action = flow.next().unwrap();
        }
        Self::start_with(&action, color, params)
    }

    pub fn start_with(
        action: &Action,
        color: ColorRgb,
        params: HookParams,
    ) -> Result<Self, HookError> {
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
                    params,
                    nodes: vec![],
                    edges,
                    now: will_be_overwritten_with_magic_ring,
                    parents: vec![],
                    labels: HashMap::new(),
                    override_previous_node: None,
                    color,
                    last_stitch: None,
                    last_mark: None,
                    mark_to_node: HashMap::new(),
                    part_limits: vec![],
                    mr_count: 0,
                };
                result.magic_ring(*x);
                Ok(result)
            }
            _ => Err(HookError::BadStarter),
        }
    }

    fn magic_ring(&mut self, size: usize) {
        assert_eq!(self.edges.last().unwrap().len(), 0);

        self.part_limits.push(self.now.cursor);

        let ring_root = self.now.cursor; // will be 0 unless using multipart
        let ring_end = ring_root + size;

        // spot for ring root in edges is already created
        self.parents.push(None); // ring root has no parent
        self.add_node(Some(Peculiarity::Locked));
        for _ in 0..size {
            self.edges.grow();
            self.parents.push(Some(ring_root));
            self.add_node(None);
        }
        self.edges.grow(); // prepare place for the next node

        // connect outer nodes to ring root
        for connected_to_root in ring_root + 1..=ring_end {
            self.edges.link(ring_root, connected_to_root);
        }
        // connect outer nodes to each other
        for outer_ring_stitch in ring_root + 1..ring_end {
            self.edges.link(outer_ring_stitch, outer_ring_stitch + 1);
        }

        self.now = Moment {
            anchors: Queue::from_iter(ring_root + 1..=ring_end),
            cursor: ring_end + 1,
            working_on: WorkingLoops::Both,
            limb_ownerhip: self.mr_count,
        };
        self.mr_count += 1;

        assert_eq!(self.edges.last().unwrap().len(), 0);
    }
}
