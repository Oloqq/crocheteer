use std::collections::HashMap;

use colors::Color;
use serde_derive::Serialize;

use crate::common::*;

#[derive(Debug, Clone)]
pub struct Edges {
    edges: Vec<Vec<usize>>,
}

impl Edges {
    #[allow(unused)]
    pub fn new() -> Self {
        Self { edges: vec![] }
    }

    #[allow(unused)] // used in tests
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

    pub fn last(&self) -> Option<&Vec<usize>> {
        self.edges.last()
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

pub struct InitialGraph {
    pub edges: Edges,
    pub peculiarities: HashMap<usize, Peculiarity>,
    pub colors: Vec<Color>,
    pub round_spans: Vec<(usize, usize)>,
    pub part_limits: Vec<usize>,
    pub mark_to_node: HashMap<String, usize>,
}

pub type PointsOnPushPlane = (usize, usize, usize);

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Peculiarity {
    Locked,
    Tip,
    BLO(PointsOnPushPlane),
    FLO(PointsOnPushPlane),
}

fn fill_round_span(edges: &Edges, round_spans: &mut Vec<(usize, usize)>) {
    let nodenum = edges.len();
    let lastspan = round_spans.last().unwrap();
    let end = lastspan.1;
    if end < nodenum - 1 {
        round_spans.push((end + 1, nodenum - 1));
    }
}

impl InitialGraph {
    pub fn from_hook(
        edges: Edges,
        peculiar: HashMap<usize, Peculiarity>,
        mut round_spans: Vec<(usize, usize)>,
        part_limits: Vec<usize>,
        colors: Vec<Color>,
        mark_to_node: HashMap<String, usize>,
    ) -> Self {
        log::trace!("round spans: {:?}", round_spans);
        fill_round_span(&edges, &mut round_spans);

        Self {
            edges,
            peculiarities: peculiar,
            colors,
            round_spans,
            mark_to_node,
            part_limits,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
