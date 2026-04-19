pub mod init;

use glam::Vec3;

use crate::{
    PlushieDef,
    data::{Edges, Node as NodeDefinition},
};

#[derive(Debug, Clone)]
pub struct SimulatedPlushie {
    /// Edges of the graph.
    edges: Edges,
    /// Nodes of the graph.
    nodes: Vec<Node>,
    /// Part data. Part stores node range, not nodes themselves.
    parts: Vec<Part>,
    /// Used with OneByOne initializer.
    one_by_one_state: Option<OneByOneState>,
    /// Basis for calculating forces
    hook_size: f32,
}

#[derive(Debug, Clone)]
pub struct Part {
    pub name: String,
    node_start: usize,
    node_end: usize,
    centroids: usize,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub definition: NodeDefinition,
    pub position: Vec3,
}

#[derive(Debug, Clone)]
pub struct OneByOneState {
    full_definition: PlushieDef,
}

impl SimulatedPlushie {
    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn edges(&self) -> &Edges {
        &self.edges
    }

    pub fn parts(&self) -> &Vec<Part> {
        &self.parts
    }
}
