use crate::common::*;

use std::{collections::HashMap, f32::consts::PI};

pub type Edges = Vec<Vec<usize>>;
pub type Nodes = Vec<Point>;

pub struct HookResult {
    pub edges: Edges,
    pub nodes: Vec<Point>,
    pub peculiarities: HashMap<usize, Peculiarity>,
    pub approximate_height: f32,
}

pub enum Peculiarity {
    Root,
    #[allow(unused)]
    BLO,
    #[allow(unused)]
    FLO,
    #[allow(unused)]
    Constrained,
}

impl HookResult {
    /// Creates and places points in initial positions
    pub fn from_hook(
        edges: Edges,
        peculiar: HashMap<usize, Peculiarity>,
        round_spans: Vec<(usize, usize)>,
    ) -> Self {
        let (nodes, highest) = make_nodes(round_spans);
        Self {
            edges,
            nodes,
            peculiarities: peculiar,
            approximate_height: highest,
        }
    }
}

fn make_nodes(round_spans: Vec<(usize, usize)>) -> (Nodes, f32) {
    // assumption: only one radial axis, how to handle shape of letter Y?
    let mut prev = 0;
    let mut y = 0.0;
    let mut nodes = vec![];

    // TODO what about the tip
    for (from, to) in round_spans {
        let count = to - from + 1;
        nodes.append(&mut ring(count, y, 1.0));
        y += 0.7;
    }

    (nodes, y)
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
    fn test_make_nodes() {
        let rounds_spans = vec![(0, 4)];
        let (res, _) = make_nodes(rounds_spans);
        assert_eq!(res.len(), 5);

        let rounds_spans = vec![(0, 4), (5, 8)];
        let (res, _) = make_nodes(rounds_spans);
        assert_eq!(res.len(), 9);
    }
}
