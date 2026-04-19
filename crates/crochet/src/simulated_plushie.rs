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
    pub fn from(
        definition: PlushieDef,
        initializer: &Initializer,
        hook_size: f32,
        part_limits: &Vec<usize>,
    ) -> Self {
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

        let parts = extract_parts(&definition, part_limits);
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

fn extract_parts(definition: &PlushieDef, part_limits: &Vec<usize>) -> Vec<Part> {
    assert_eq!(part_limits.len(), definition.pattern.parts.len());
    let mut last_end = 0;
    let mut limits = part_limits.iter();

    let parts: Vec<Part> = definition
        .pattern
        .parts
        .iter()
        .map(|part_def| {
            let previous_end = last_end;
            last_end = *limits.next().unwrap();
            Part {
                name: part_def.name.clone(),
                node_start: previous_end,
                node_end: last_end,
                centroids: part_def.parameters.centroids,
            }
        })
        .collect();

    assert_eq!(parts.len(), definition.pattern.parts.len());

    parts
}
