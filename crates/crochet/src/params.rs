use std::{collections::HashMap, error::Error};

#[derive(Debug, PartialEq, Clone)]
pub struct Params {
    /// Multiplier to all forces in a single step
    pub timestep: f32,
    /// Set to true if creation is meant to stand on it's own to simulate a flat bottom
    /// Set to false if the creation is carried around, so that the bottom is not flat
    pub floor: bool,
    /// Force that pulls the nodes down
    pub gravity: f32,
    /// Force acting between each two nodes
    pub node_to_node: f32,
    /// Distance between nodes that is considered "relaxed"
    pub desired_stitch_distance: f32,
    /// Configuration of centroid stuffing
    pub centroids: CentroidParams,
    /// if true, the whole shape will be translated by displacement of root, so that root stays at (0, 0, 0).
    /// not applicable to LegacyPlushie
    pub reflect_locked: bool,
    /// Multipler for BLO/FLO force. If BLO/FLO behaves incorrectly, probably the sign is wrong.
    /// I assume it has to do with working the plushie clockwise vs counterclockwise.
    /// It has yet to be investigated.
    pub single_loop_force: f32,
    /// Method for setting initial positions of stitches
    pub initializer: Initializer,
    /// Required displacement on a node for it to be affected. (Displacements with maginute below the threshold will be ignored)
    pub minimum_displacement: f32,

    pub hook: HookParams,

    pub track_performance: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HookParams {
    pub tip_from_fo: bool,
    pub enforce_counts: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CentroidParams {
    /// Number of centroids that simulate the stuffing. More centroids = more internal pressure. Bigger shapes need more.
    pub number: usize,
    pub force: f32,
    pub min_nodes_per_centroid: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Initializer {
    /// Start with a few stitches, and build the plushie while simulation is running.
    OneByOne(OneByOneParams),
    /// Start with points arranged roughly in the shape of a cylinder
    Cylinder,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OneByOneParams {
    /// Plushie will wait with expansion until the previous node is stabilized.
    /// Parameter sets the maximum displacement where the next node shall be added.
    pub acceptable_displacement_for_expanding: f32,
    /// If previous node cannot be stabilized, next one shall be added after set time.
    pub force_expansion_after_time: f32,
}

impl Params {
    pub fn unconstrained_floating() -> Self {
        Self {
            timestep: 1.0,
            centroids: Default::default(),
            floor: false,
            gravity: 0.003,
            node_to_node: 0.0,
            desired_stitch_distance: 1.0,
            reflect_locked: false,
            single_loop_force: 0.05,
            initializer: Initializer::Cylinder,
            minimum_displacement: 0.001,
            track_performance: false,
            hook: Default::default(),
        }
    }

    pub fn floored() -> Self {
        Self {
            floor: true,
            reflect_locked: true,
            ..Self::unconstrained_floating()
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::floored()
    }
}

impl Default for HookParams {
    fn default() -> Self {
        Self {
            tip_from_fo: false,
            enforce_counts: true,
        }
    }
}

impl Default for CentroidParams {
    fn default() -> Self {
        Self {
            number: 2,
            force: 0.05,
            min_nodes_per_centroid: 60,
        }
    }
}

impl Default for OneByOneParams {
    fn default() -> Self {
        Self {
            acceptable_displacement_for_expanding: 0.03,
            force_expansion_after_time: 100.0,
        }
    }
}
