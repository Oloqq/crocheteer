use super::{Hook, Queue};
use std::collections::HashMap;

use crate::{
    ColorRgb,
    acl::Origin,
    data::Peculiarity,
    graph_construction::hook::{Edges, HookParams, Moment, WorkingLoops},
};

const DEFAULT_COLOR: ColorRgb = [255, 0, 255];

impl Hook {
    pub fn new(params: HookParams) -> Self {
        Self {
            params,
            nodes: vec![],
            edges: Edges::new(),
            now: Moment {
                cursor: 0,
                anchors: Default::default(),
                working_on: WorkingLoops::Both,
                part: 0,
            },
            labels: HashMap::new(),
            override_previous_node: None,
            color: DEFAULT_COLOR,
            last_stitch: None,
            last_mark: None,
            mark_to_node: HashMap::new(),
            part_limits: vec![],
            part_cursor: 0,
        }
    }

    pub(super) fn magic_ring(&mut self, size: usize, origin: Option<Origin>) {
        assert_eq!(self.edges.last().unwrap().len(), 0);

        let ring_root = self.now.cursor; // will be 0 unless using multipart
        let ring_end = ring_root + size;

        // spot for ring root in edges is already created
        self.add_node(origin).peculiarity(Peculiarity::Locked);
        for _ in 0..size {
            self.add_node(origin).parent(ring_root);
        }

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
            part: self.part_cursor,
        };
        self.part_cursor += 1;

        assert_eq!(self.edges.last().unwrap().len(), 0);
    }
}
