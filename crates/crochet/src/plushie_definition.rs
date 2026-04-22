use crate::{
    acl::PatternAst,
    data::{DeferredEdge, Edges, Node, PartClusters},
};

pub type ColorRgb = [u8; 3];

#[derive(Debug, Clone)]
pub struct PlushieDef {
    /// Abstract Syntax Tree of the pattern
    pub pattern: PatternAst,
    /// Edges of the graph
    /// For every edges[i], each element of edges[i] is smaller than i. This is important to easily manage partially built plushies.
    pub edges: Edges,
    pub nodes: Vec<Node>,
    pub part_clusters: PartClusters,
    pub deferred_edges: Vec<DeferredEdge>,
}
