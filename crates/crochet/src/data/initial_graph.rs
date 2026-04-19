use std::collections::HashMap;

use crate::data::{Edges, Node};

#[derive(Debug)]
pub(crate) struct InitialGraph {
    pub edges: Edges,
    pub nodes: Vec<Node>,
    pub part_limits: Vec<usize>,
    #[allow(dead_code)]
    pub mark_to_node: HashMap<String, usize>,
}
