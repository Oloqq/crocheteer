use std::collections::HashMap;

use crate::{ColorRgb, hook::edges::Edges};

#[derive(Debug)]
pub struct InitialGraph {
    pub edges: Edges,
    pub peculiarities: HashMap<usize, Peculiarity>,
    pub colors: Vec<ColorRgb>,
    pub part_limits: Vec<usize>,
    pub mark_to_node: HashMap<String, usize>,
}

pub type PointsOnPushPlane = (usize, usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub enum Peculiarity {
    Locked,
    Tip,
    BLO(PointsOnPushPlane),
    FLO(PointsOnPushPlane),
}
