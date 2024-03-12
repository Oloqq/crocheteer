#![allow(unused)]

use crate::common::*;

use std::collections::HashMap;

use super::hook::Hook;

pub type Edges = Vec<Vec<usize>>;
pub type Nodes = Vec<Point>;

pub struct Graph {
    pub edges: Edges,
    pub nodes: Vec<Point>,
    pub peculiarities: HashMap<usize, Peculiarity>,
    pub approximate_height: f32,
}

pub enum Peculiarity {
    Root,
    BLO,
    FLO,
    Constrained,
}

impl Graph {
    /// Repositions all *peculiar* points to beginning of the vector.
    /// Creates and places points in initial positions
    pub fn new(hook: Hook) -> Self {
        todo!();
        // self
    }
}
