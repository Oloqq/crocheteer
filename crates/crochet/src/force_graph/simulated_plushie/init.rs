use glam::Vec3;

use crate::{
    PlushieDef,
    acl::Action,
    data::{Edges, PartClusters},
    force_graph::{
        Initializer,
        initializers::ring,
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
            Initializer::OneByOne => Some(OneByOneState {
                full_definition: definition.clone(),
            }),
        };

        let (parts, part_clusters) = extract_parts(&definition, part_limits, initializer);
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
        let (edges, deferred_edges) = match initializer {
            Initializer::RegularCylinder(_) => {
                let mut edges = definition.edges;
                for edge in definition.deferred_edges {
                    edges.link(edge.node_a, edge.node_b);
                }
                (edges, vec![])
            }
            Initializer::OneByOne => (
                Edges::from_trimmed(definition.edges, nodes.len()),
                definition.deferred_edges.into_iter().rev().collect(),
            ),
        };
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
            part_clusters,
            deferred_edges,
            one_by_one_state,
            hook_size,
            tensions,
        }
    }

    pub fn advance_one_by_one(&mut self) -> OneByOneResult {
        let Some(obo) = &self.one_by_one_state else {
            return OneByOneResult::Noop;
        };
        let new_index = self.nodes.len();

        while let Some(index) = self.part_clusters.index_of_next_join()
            && new_index >= index
        {
            self.part_clusters.perform_next_join();
            for i in 0..self.parts.len() {
                let cluster = self.part_clusters.get_part_cluster(i);
                let reflecting_node = self.parts[cluster].start.clone();
                self.parts[i].reflecting_node = Some(reflecting_node);
            }
        }

        if let Some(deferred_edge) = self.deferred_edges.pop_if(|l| new_index >= l.with_node) {
            let (a, b) = (deferred_edge.node_a, deferred_edge.node_b);
            self.edges.link(a, b);
            self.tensions[a.max(b)].push(0.0);
            return OneByOneResult::CreatedEdge(a, b);
        }

        if new_index >= obo.full_definition.nodes.len() {
            self.one_by_one_state = None;
            return OneByOneResult::JustFinished;
        }
        assert!(self.nodes.len() == self.edges.len());
        let definition = &obo.full_definition.nodes[new_index];

        match definition.origin.action {
            Action::MR(size) => self.import_magic_ring(new_index, size),
            _ => self.import_one_node(new_index),
        }
    }

    fn import_one_node(&mut self, new_index: usize) -> OneByOneResult {
        let obo = &self
            .one_by_one_state
            .as_ref()
            .expect("this should be reachable only with obo");
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
        let definition = obo.full_definition.nodes[new_index].clone();

        self.nodes.push(Node {
            definition,
            position,
            rooted: false,
        });
        OneByOneResult::CreatedNode(new_index)
    }

    fn import_magic_ring(&mut self, start_index: usize, count: usize) -> OneByOneResult {
        let obo = &self
            .one_by_one_state
            .as_ref()
            .expect("this should be reachable only with obo");

        let mut positions = vec![Vec3::ZERO];
        positions.append(&mut ring(count as u32, self.hook_size, self.hook_size));
        assert_eq!(self.edges.len(), start_index);
        // + 1 for virtual
        for i in 0..count + 1 {
            self.edges.clone_next_node(&obo.full_definition.edges);
            self.tensions
                .push(vec![0.0; self.edges.last().unwrap().len()]);
            self.nodes.push(Node {
                definition: obo.full_definition.nodes[start_index + i].clone(),
                position: positions[i],
                rooted: false,
            });
        }
        assert_eq!(self.edges.len(), start_index + count + 1);

        OneByOneResult::CreatedMagicRing {
            start: start_index,
            count: count + 1, // +1 for virtual
        }
    }
}

pub enum OneByOneResult {
    /// Created one node at given index.
    CreatedNode(usize),
    /// Created a magic ring.
    CreatedMagicRing { start: usize, count: usize },
    /// Created a link (graph edge) between two nodes.
    CreatedEdge(usize, usize),
    // Waiting, // TODO wait until previous node is relatively stable (configurable)
    /// The previous call produced the last node. No new node was produced with this call.
    JustFinished,
    /// One by one initialization is already finished.
    Noop,
}

fn new_node_position(based_on: &Vec<Vec3>, hook_size: f32) -> Vec3 {
    if based_on.len() == 0 {
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

fn extract_parts(
    definition: &PlushieDef,
    part_limits: &Vec<usize>,
    initializer: &Initializer,
) -> (Vec<Part>, PartClusters) {
    assert_eq!(part_limits.len(), definition.pattern.parts.len());
    let mut end = 0;
    let mut limits = part_limits.iter();

    let mut parts: Vec<Part> = definition
        .pattern
        .parts
        .iter()
        .map(|part_def| {
            let start = end;
            end = *limits.next().unwrap();
            Part {
                name: part_def.name.clone(),
                start,
                end,
                centroids_wanted: part_def.parameters.centroids,
                centroids: vec![],
                reflecting_node: Some(start),
            }
        })
        .collect();

    assert_eq!(parts.len(), definition.pattern.parts.len());

    let mut clusters = definition.part_clusters.clone();
    match initializer {
        Initializer::RegularCylinder(_) => {
            clusters.perform_all_joins();
            for i in 0..parts.len() {
                let cluster = clusters.get_part_cluster(i);
                let reflecting_node = parts[cluster].start.clone();
                parts[i].reflecting_node = Some(reflecting_node);
            }
        }
        Initializer::OneByOne => (),
    };

    (parts, clusters)
}
