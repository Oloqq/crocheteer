use crate::common::*;

use std::{collections::HashMap, f32::consts::PI};

pub type Edges = Vec<Vec<usize>>;
pub type Nodes = Vec<Point>;

pub struct Graph {
    pub edges: Edges,
    pub nodes: Vec<Point>,
    pub peculiarities: HashMap<usize, Peculiarity>,
    pub approximate_height: f32,
}

#[allow(unused)]
pub enum Peculiarity {
    Root,
    BLO,
    FLO,
    Constrained,
}

impl Graph {
    /// Creates and places points in initial positions
    pub fn from_hook(
        edges: Edges,
        peculiar: HashMap<usize, Peculiarity>,
        round_starts: Vec<usize>,
    ) -> Self {
        let (nodes, highest) = make_nodes(round_starts);
        Self {
            edges,
            nodes,
            peculiarities: peculiar,
            approximate_height: highest,
        }
    }
}

fn make_nodes(round_starts: Vec<usize>) -> (Nodes, f32) {
    // assumption: only one radial axis, how to handle shape of letter Y?
    let mut prev = 0;
    let mut y = 0.0;
    let mut nodes = vec![];

    // TODO what about the tip
    for rstart in round_starts {
        let count = rstart - prev;

        nodes.append(&mut ring(count, y, 1.0));
        y += 0.7;

        prev = rstart;
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
        let rs = vec![4];
        let (res, _) = make_nodes(rs);
        assert_eq!(res.len(), 4);

        let rs = vec![4, 8];
        let (res, _) = make_nodes(rs);
        assert_eq!(res.len(), 8);
    }
}
