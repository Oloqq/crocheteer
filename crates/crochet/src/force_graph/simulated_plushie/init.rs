use glam::Vec3;

use crate::{
    PlushieDef,
    data::Edges,
    force_graph::{
        Initializer,
        simulated_plushie::{Node, OneByOneState, Part},
    },
};

impl super::SimulatedPlushie {
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
        let nodes: Vec<Node> = definition
            .nodes
            .into_iter()
            .zip(node_positions)
            .map(|(def, pos)| Node {
                definition: def,
                position: pos,
                rooted: false,
            })
            .collect();
        let edges = Edges::from_trimmed(definition.edges, nodes.len());
        let tensions = edges
            .data()
            .iter()
            .map(|edgelist| vec![0.0; edgelist.len()])
            .collect();

        Self {
            displacement: vec![Vec3::ZERO; nodes.len()],
            edges,
            nodes,
            parts,
            one_by_one_state,
            hook_size,
            tensions,
        }
    }

    pub fn advance_one_by_one(&mut self) -> OneByOneResult {
        let Some(obo) = &self.one_by_one_state else {
            return OneByOneResult::Finished;
        };

        assert!(self.nodes.len() == self.edges.len());
        let new_index = self.nodes.len();
        if new_index >= obo.full_definition.nodes.len() {
            self.one_by_one_state = None;
            return OneByOneResult::Finished;
        }

        self.edges.clone_next_node(&obo.full_definition.edges);
        self.tensions
            .push(vec![0.0; self.edges.last().unwrap().len()]);
        assert_eq!(self.edges.len(), new_index + 1);
        let position_basis: Vec<Vec3> = self
            .edges
            .edges_from_node(new_index)
            .iter()
            .map(|e| self.nodes[*e].position)
            .collect();
        let position = new_node_position(&position_basis, self.hook_size);

        self.nodes.push(Node {
            definition: obo.full_definition.nodes[new_index].clone(),
            position,
            rooted: false,
        });

        OneByOneResult::Advanced(new_index)
    }
}

pub enum OneByOneResult {
    Advanced(usize),
    // Waiting, // TODO wait until previous node is relatively stable (configurable)
    Finished,
}

fn new_node_position(based_on: &Vec<Vec3>, hook_size: f32) -> Vec3 {
    if based_on.len() == 0 {
        // FIXME fails with unconnected parts
        unreachable!()
    } else if based_on.len() == 1 {
        based_on[0] + Vec3::new(0.0, hook_size, 0.0)
    } else {
        let mut avg = Vec3::ZERO;
        for base in based_on {
            avg += base;
        }
        avg /= based_on.len() as f32;
        // TODO addition of HOOK_SIZE to Y can behave weird if work transitions from building vertically to horizontally
        // this is needed for now to introduce variation third dimension, otherwise nodes settle on a plane
        // ideally, implementation would be completely agnostic to orientation
        // the "working horizontally" thing could be solved by using vector from parent to current node here
        // the issue of introducing third dimension still needs to be addressed then
        avg += Vec3::new(0.0, hook_size, 0.0);
        avg
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
                start: previous_end,
                end: last_end,
                centroids_wanted: part_def.parameters.centroids,
                centroids: vec![Vec3::ZERO], // TEMP
            }
        })
        .collect();

    assert_eq!(parts.len(), definition.pattern.parts.len());

    parts
}
