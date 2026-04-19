use glam::Vec3;

use crate::{
    PlushieDef,
    data::{Edges, Node as NodeDefinition},
    force_graph::Initializer,
};

pub struct SimulatedPlushie {
    /// Edges of the graph.
    edges: Edges,
    /// Nodes of the graph.
    nodes: Vec<Node>,
    /// Part data. Part stores node range, not nodes themselves.
    parts: Vec<Part>,
    /// Used with OneByOne initializer.
    one_by_one_state: Option<OneByOneState>,
}

pub struct Part {
    pub name: String,
    node_start: usize,
    node_end: usize,
    centroids: usize,
}

pub struct Node {
    pub definition: NodeDefinition,
    pub position: Vec3,
}

pub struct OneByOneState {
    full_definition: PlushieDef,
}

impl SimulatedPlushie {
    pub fn from(definition: PlushieDef, initializer: &Initializer, hook_size: f32) -> Self {
        assert!(definition.nodes.len() == definition.edges.len());

        let node_positions = initializer.apply(definition.nodes.len() as u32, hook_size);

        let one_by_one_state = match initializer {
            Initializer::RegularCylinder(_) => {
                assert_eq!(node_positions.len(), definition.nodes.len());
                None
            }
            Initializer::OneByOne => {
                assert!(node_positions.len() >= 3);
                Some(OneByOneState {
                    full_definition: definition.clone(),
                })
            }
        };

        let parts = extract_parts(&definition);
        let nodes = definition
            .nodes
            .into_iter()
            .zip(node_positions)
            .map(|(def, pos)| Node {
                definition: def,
                position: pos,
            })
            .collect();

        Self {
            edges: definition.edges,
            nodes,
            parts,
            one_by_one_state,
        }
    }

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

fn extract_parts(definition: &PlushieDef) -> Vec<Part> {
    let mut parts: Vec<Part> = definition
        .pattern
        .parts
        .iter()
        .map(|part_def| Part {
            name: part_def.name.clone(),
            node_start: 0,
            node_end: 0,
            centroids: part_def.parameters.centroids,
        })
        .collect();

    // TODO use part limits from hook
    for (i, node) in definition.nodes.iter().enumerate() {
        assert!(node.part_index <= parts.len());
        if parts[node.part_index].node_start == 0 {
            parts[node.part_index].node_start = i;
        }
        if parts[node.part_index].node_end < i {
            parts[node.part_index].node_end = i;
        }
    }

    assert_eq!(parts.len(), definition.pattern.parts.len());

    parts
}
