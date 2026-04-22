mod adding_nodes;
mod attaching;
mod mark_and_goto;
mod part_joiner;
mod perform;
mod starters;
mod stitch_builder;

use crate::{
    ColorRgb,
    acl::{
        Action::{self},
        Label,
    },
    data::{DeferredEdge, Edges, InitialGraph, Node, PartClusters},
    graph_construction::{errors::ErrorCode, hook::part_joiner::PartJoiner},
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
    /// Index of the part it is working on
    part: usize,
}

impl Default for Moment {
    fn default() -> Self {
        Self {
            cursor: 0,
            anchors: Default::default(),
            working_on: WorkingLoops::Both,
            part: 0,
        }
    }
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
    // TODO this potentially would be not needed if mark_ahead was implemented (was used for attach)
    /// Was the last action a mark?
    last_mark: Option<Action>,
    /// Map from labels to the index of the node they are on.
    mark_to_node: HashMap<Label, usize>,
    /// Node indexes where parts begin and end. When Hook finishes, first element should be equal to zero, last element should be equal to colors.len()
    part_limits: Vec<usize>,
    /// Part currently in construction.
    part_cursor: usize,
    /// Used to track how parts are joined.
    part_joins: PartJoiner,
    /// Edges that for purposes of OneByOne initializer should not be immediately added to the graph.
    deferred_edges: Vec<DeferredEdge>,
}

impl Hook {
    pub(crate) fn finish(mut self) -> InitialGraph {
        self.edges.cleanup();
        let part_count = self.part_limits.len();
        InitialGraph {
            edges: self.edges,
            nodes: self.nodes,
            mark_to_node: self.mark_to_node,
            part_limits: self.part_limits,
            part_joins: PartClusters::new(part_count, self.part_joins.take()),
            deferred_edges: self.deferred_edges,
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
