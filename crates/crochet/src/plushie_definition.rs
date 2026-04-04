pub type ColorRgb = [u8; 3];
pub type Edges = Vec<Vec<usize>>;

pub struct Node {
    pub color: ColorRgb,
}

pub struct PlushieDef {
    /// Edges of the graph
    /// For every edges[i], each element of edges[i] is smaller than i. This is important to easily manage partially built plushies.
    pub edges: Edges,
    pub nodes: Vec<Node>,
}
