pub use crate::hook::hook_result::InitialGraph;
use crate::{
    acl::pest_parser::Pattern,
    hook::{Hook, HookParams},
};

#[allow(unused)] // TODO
mod acl;
mod force_graph;
#[allow(unused)] // TODO
mod hook;
#[allow(unused)] // TODO
mod params;

pub use force_graph::{centroid_stuffing, link_force_magnitude, link_forces};
use glam::Vec3;

pub fn parse(acl_source: &str) -> Option<PlushieDef> {
    let Ok(syntax_result) = Pattern::parse(acl_source) else {
        return None;
    };
    // println!("syntax: {:?}", syntax_result);
    let Ok(semantic_result) = Hook::parse(syntax_result, &HookParams::default()) else {
        return None;
    };
    // println!("semantic: {:?}", semantic_result);
    let hook_size = 5e-4;
    let initializer = force_graph::initializers::Initializer::RegularCylinder(12);
    let nodes = initializer.apply(semantic_result.edges.len() as u32, hook_size);

    Some(PlushieDef {
        nodes,
        edges: semantic_result.edges.into(),
    })
}

pub type Edges = Vec<Vec<usize>>;

pub struct PlushieDef {
    // TODO produce this once at function call to initializer
    // PlushieDef does not need to know anything about node positions
    pub nodes: Vec<Vec3>,
    /// Edges of the graph
    /// For every edges[i], each element of edges[i] is smaller than i. This is important to easily manage partially built plushies.
    pub edges: Edges,
}
