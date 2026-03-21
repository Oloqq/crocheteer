use std::f32::consts::PI;

use super::hook_result::InitialGraph;
use crate::{
    common::*,
    plushie::{animated::nodes::Nodes, params::Initializer},
};

impl Initializer {
    pub fn apply_to(
        &self,
        graph: InitialGraph,
    ) -> (Nodes, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<V>) {
        let nodes = Nodes::new(
            match self {
                Initializer::OneByOne(_) => vec![],
                Initializer::Cylinder => arrange_cylinder(graph.colors.len()),
            },
            graph.peculiarities,
            graph.colors,
        );

        let edges: Vec<Vec<usize>>;
        let edges_goal: Vec<Vec<usize>>;
        let mut displacement = Vec::with_capacity(graph.edges.len());
        match self {
            Initializer::OneByOne(_) => {
                edges = vec![];
                edges_goal = graph.edges.into();
                displacement.push(V::zeros());
            }
            Initializer::Cylinder => {
                edges = graph.edges.into();
                edges_goal = edges.clone();
                displacement = vec![V::zeros(); edges.len()];
            }
        }

        (nodes, edges, edges_goal, displacement)
    }
}

fn arrange_cylinder(node_num: usize) -> Vec<Point> {
    assert!(node_num > 0);

    let mut y = 0.0;
    let mut nodes = vec![Point::new(0.0, 0.0, 0.0)]; // for the magic ring
    let mut included = 1;
    let batch = 12;

    while node_num - included > batch {
        nodes.append(&mut ring(batch, y, 1.0));
        y += 1.0;
        included += batch;
    }
    nodes.append(&mut ring(node_num - included, y, 1.0));
    nodes
}

fn ring(nodes: usize, y: f32, desired_stitch_distance: f32) -> Vec<Point> {
    let circumference = (nodes + 1) as f32 * desired_stitch_distance * 5.0;
    let radius = circumference / (2.0 * PI) / 4.0;

    let interval = 2.0 * PI / nodes as f32;
    let mut result: Vec<Point> = vec![];

    for i in 0..nodes {
        let rads = interval * i as f32;
        let x = rads.cos() * radius;
        let z = rads.sin() * radius;
        let point = Point::new(x, y, z);
        result.push(point);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrange_cylinder() {
        let res = arrange_cylinder(5);
        assert_eq!(res.len(), 5);

        let res = arrange_cylinder(9);
        assert_eq!(res.len(), 9);

        let res = arrange_cylinder(21);
        assert_eq!(res.len(), 21);

        let res = arrange_cylinder(12);
        assert_eq!(res.len(), 12);
    }
}
