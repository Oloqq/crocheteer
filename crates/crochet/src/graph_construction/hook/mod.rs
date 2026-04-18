mod adding_nodes;
mod attaching;
mod mark_and_goto;
mod perform;
mod starters;
mod stitch_builder;

use crate::{
    ColorRgb,
    acl::{
        Action::{self},
        Label,
    },
    data::{Edges, InitialGraph, Node},
    graph_construction::errors::ErrorCode,
};
use std::collections::HashMap;
pub use std::collections::VecDeque as Queue;
use stitch_builder::StitchBuilder;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct HookParams {
    // in the poc, FO in a round of too many nodes could break the simulation
    // does it still happen? can it be prevented (resilient implementation or hook error)?
    // should this parameter be exposed to CAD?
    pub tip_from_fo: bool,
    pub enforce_counts: bool,
}

#[derive(Clone, Debug)]
pub enum WorkingLoops {
    Both,
    Back,
    Front,
}

/// Context of hook working at given cursor
#[derive(Clone, Debug)]
struct Moment {
    /// Node index to be created
    cursor: usize,
    anchors: Queue<usize>,
    working_on: WorkingLoops,
    /// Moments on unconnected graphs shall have different number
    limb_ownerhip: usize,
}

/// Responsible for building the graph used in the simulation
#[derive(Clone, Debug)]
pub struct Hook {
    params: HookParams,
    nodes: Vec<Node>,
    edges: Edges,
    now: Moment,
    /// Storage of spots for Mark and Goto
    labels: HashMap<Label, Moment>,
    /// Current color/yarn. Not stored in Moment as typically yarn changes happpen independently of switching positions.
    color: ColorRgb,
    // Previous stitch might need to be overwritten after a Goto
    override_previous_node: Option<usize>,
    /// Last stitch created (not counting actions like mark, goto)
    last_stitch: Option<Action>,
    // TODO this potentially would be not needed if mark_ahead was implemented
    /// Was the last action a mark?
    last_mark: Option<Action>,
    /// Map from labels to the index of the node they are on.
    mark_to_node: HashMap<Label, usize>,
    /// Indexes where parts begin and end. When Hook finishes, first element should be equal to zero, last element should be equal to colors.len()
    part_limits: Vec<usize>,
    /// Used to track unconnected limbs
    magic_ring_count: usize,
}

impl Hook {
    pub(crate) fn finish(mut self) -> InitialGraph {
        self.edges.cleanup();
        InitialGraph {
            edges: self.edges,
            nodes: self.nodes,
            mark_to_node: self.mark_to_node,
            part_limits: self.part_limits,
        }
    }

    fn previous_stitch(&mut self) -> usize {
        match self.override_previous_node {
            Some(x) => {
                self.override_previous_node = None;
                x
            }
            None => self.now.cursor - 1,
        }
    }
}

#[cfg(test)]
mod tests;
