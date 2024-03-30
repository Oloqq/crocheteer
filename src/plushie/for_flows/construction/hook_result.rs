use serde_derive::Serialize;

use crate::common::*;

use std::{collections::HashMap, f32::consts::PI};

#[derive(Debug, Clone)]
pub struct Edges {
    edges: Vec<Vec<usize>>,
}

impl Edges {
    // #[allow(unused)]
    pub fn new() -> Self {
        Self { edges: vec![] }
    }

    // #[allow(unused)] // used in tests
    pub fn from_unchecked(ordered: Vec<Vec<usize>>) -> Self {
        Self { edges: ordered }
    }

    pub fn from(mby_unordered: Vec<Vec<usize>>) -> Self {
        let mut res = Self {
            edges: vec![vec![]; mby_unordered.len()],
        };
        for (node1, destinations) in mby_unordered.into_iter().enumerate() {
            for node2 in destinations {
                res.link_no_checks(node1, node2);
            }
        }
        res
    }

    fn link_no_checks(&mut self, node1: usize, node2: usize) {
        if node2 > node1 {
            self.edges[node2].push(node1);
        } else {
            self.edges[node1].push(node2);
        }
    }

    pub fn link(&mut self, node1: usize, node2: usize) {
        assert!(node1 != node2, "Node can't link to itself");
        assert!(
            node1 < self.edges.len() && node2 < self.edges.len(),
            "Both nodes to link should already have their spots in Edges ({}, {}, len: {})",
            node1,
            node2,
            self.edges.len()
        );
        self.link_no_checks(node1, node2);
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn grow(&mut self) {
        self.edges.push(Vec::with_capacity(2));
    }

    pub fn cleanup(&mut self) {
        assert!(self.edges.last().unwrap().len() == 0);
        self.edges.pop();
    }
}

impl Into<Vec<Vec<usize>>> for Edges {
    fn into(self) -> Vec<Vec<usize>> {
        self.edges
    }
}

impl PartialEq for Edges {
    fn eq(&self, other: &Self) -> bool {
        use std::collections::HashSet;

        if self.edges.len() != other.edges.len() {
            return false;
        }

        for (my, their) in self.edges.iter().zip(&other.edges) {
            let myset: HashSet<usize> = HashSet::from_iter(my.iter().cloned());
            let theirset: HashSet<usize> = HashSet::from_iter(their.iter().cloned());
            if myset != theirset {
                return false;
            }
        }

        return true;
    }
}

pub type Nodes = Vec<Point>;

pub struct HookResult {
    pub edges: Edges,
    pub nodes: Vec<Point>,
    pub peculiarities: HashMap<usize, Peculiarity>,
    pub approximate_height: f32,
    pub colors: Vec<Color>,
}

pub type PointsOnPushPlane = (usize, usize, usize);

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Peculiarity {
    Root,
    Tip,
    BLO(PointsOnPushPlane),
    FLO(PointsOnPushPlane),
    Constrained(V),
}

impl HookResult {
    /// Creates and places points in initial positions
    pub fn from_hook(
        edges: Edges,
        peculiar: HashMap<usize, Peculiarity>,
        round_spans: Vec<(usize, usize)>,
        colors: Vec<Color>,
    ) -> Self {
        log::debug!("round spans: {:?}", round_spans);
        let (nodes, highest) = make_nodes(round_spans);
        Self {
            edges,
            nodes,
            peculiarities: peculiar,
            approximate_height: highest,
            colors,
        }
    }
}

fn make_nodes(round_spans: Vec<(usize, usize)>) -> (Nodes, f32) {
    // assumption: only one radial axis, how to handle shape of letter Y?
    let mut y = 0.0;
    let mut nodes = vec![];

    println!("{:?}", round_spans);
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

    #[test]
    fn test_edges_from() {
        let src = vec![
            vec![1, 2, 3], // 0
            vec![2, 4],    // 1
            vec![3, 5],    // 2
            vec![4, 6],    // 3
            vec![5],       // 4
            vec![6],       // 5
            vec![],        // 6
        ];
        let e = Edges::from(src);
        assert_eq!(
            e.edges,
            vec![
                vec![],     // 0
                vec![0],    // 1
                vec![0, 1], // 2
                vec![0, 2], // 3
                vec![1, 3], // 4
                vec![2, 4], // 5
                vec![3, 5], // 6
            ]
        )
    }
}
