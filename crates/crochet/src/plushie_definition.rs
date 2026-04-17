// use std::collections::HashMap;

use crate::{Node, PatternAst};

pub type ColorRgb = [u8; 3];
pub type Edges = Vec<Vec<usize>>;

// pub struct Part {
//     pub name: String,
//     pub centroids: usize,
// }

pub struct PlushieDef {
    /// Abstract Syntax Tree of the pattern
    pub pattern: PatternAst,
    /// Edges of the graph
    /// For every edges[i], each element of edges[i] is smaller than i. This is important to easily manage partially built plushies.
    pub edges: Edges,
    pub nodes: Vec<Node>,
    // pub parts: Vec<Part>,
}
