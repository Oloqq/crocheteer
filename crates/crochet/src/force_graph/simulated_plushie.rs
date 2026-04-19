pub mod init;
pub mod step;

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
    pub parts: Vec<Part>,
    /// Used with OneByOne initializer.
    one_by_one_state: Option<OneByOneState>,
    /// Basis for calculating forces.
    hook_size: f32,
    /// Displacement buffer to avoid reallocation every step.
    displacement: Vec<Vec3>,
    /// Edge tension buffer to avoid reallocation every step. Mirrors structure of edges.
    tensions: Vec<Vec<f32>>,
}

#[derive(Debug, Clone)]
pub struct Part {
    pub name: String, // TODO pub?
    /// First index in nodes that belongs to this part.
    start: usize,
    /// Index after the last node that belongs to this part.
    end: usize,
    /// Centroids requested for this part.
    pub centroids_wanted: usize, // TODO UI updating this
    /// Centroids positions.
    centroids: Vec<Vec3>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub definition: NodeDefinition,
    pub position: Vec3,
    pub rooted: bool,
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

    pub fn root_node_at(&mut self, index: usize, pos: Vec3) {
        self.nodes[index].rooted = true;
        self.nodes[index].position = pos;
    }

    pub fn unroot_node(&mut self, index: usize) {
        self.nodes[index].rooted = false;
    }

    pub fn get_centroids(&self) -> Vec<Vec3> {
        self.parts
            .iter()
            .flat_map(|p| p.centroids.clone())
            .collect()
    }

    pub fn get_tensions(&self) -> &Vec<Vec<f32>> {
        &self.tensions
    }
}

// TODO this is just for UI, UI has to manage without it
impl Part {
    pub fn mock(name: String) -> Self {
        Self {
            name,
            start: 0,
            end: 0,
            centroids_wanted: 0,
            centroids: vec![],
        }
    }
}
