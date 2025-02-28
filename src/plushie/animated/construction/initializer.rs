use std::{collections::HashSet, f32::consts::PI};

use super::hook_result::InitialGraph;
use crate::{
    common::*,
    plushie::{animated::nodes::Nodes, params::Initializer},
    sanity,
};

impl Initializer {
    pub fn apply_to(
        &self,
        graph: InitialGraph,
    ) -> (Nodes, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<V>) {
        let nodes = Nodes::new(
            match self {
                Initializer::OneByOne(_) => vec![],
                Initializer::Cylinder => arrange_cylinder(graph.round_spans),
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

fn arrange_cylinder(round_spans: Vec<(usize, usize)>) -> Vec<Point> {
    fn is_uniq(vec: &Vec<Point>) -> bool {
        let uniq = vec
            .into_iter()
            .map(|v| format!("{:?}", v.coords))
            .collect::<HashSet<_>>();
        uniq.len() == vec.len()
    }

    let mut y = 0.0;
    let mut nodes = vec![];

    let mut round_spans = round_spans.into_iter();
    let mr_round = round_spans.next().unwrap();
    assert_eq!(mr_round.0, 0);
    nodes.append(&mut vec![Point::new(0.0, 0.0, 0.0)]);
    nodes.append(&mut ring(mr_round.1 - mr_round.0, y, 1.0));
    y += 0.7;

    for (from, to) in round_spans {
        let count = to - from + 1;
        nodes.append(&mut ring(count, y, 1.0));
        y += 0.7;
    }

    sanity!(assert!(is_uniq(&nodes), "hook created duplicate positions"));

    nodes
}

fn ring(nodes: usize, y: f32, desired_stitch_distance: f32) -> Vec<Point> {
    let circumference = (nodes + 1) as f32 * desired_stitch_distance;
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
        let rounds_spans = vec![(0, 4)];
        let res = arrange_cylinder(rounds_spans);
        assert_eq!(res.len(), 5);

        let rounds_spans = vec![(0, 4), (5, 8)];
        let res = arrange_cylinder(rounds_spans);
        assert_eq!(res.len(), 9);
    }
}
