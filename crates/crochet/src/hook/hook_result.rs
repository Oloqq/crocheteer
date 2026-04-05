use std::collections::HashMap;

use crate::hook::{edges::Edges, node::Node};

#[derive(Debug)]
pub struct InitialGraph {
    pub edges: Edges,
    pub nodes: Vec<Node>,
    pub part_limits: Vec<usize>,
    pub mark_to_node: HashMap<String, usize>,
}
