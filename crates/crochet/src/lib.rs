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
mod plushie_definition;

pub use force_graph::{
    centroid_push_magnitude, centroid_stuffing, initializers::Initializer, link_force_magnitude,
    link_forces, weight,
};
pub use plushie_definition::*;

pub fn parse(acl_source: &str) -> Option<PlushieDef> {
    let Ok(syntax_result) = Pattern::parse(acl_source) else {
        return None;
    };
    // println!("syntax: {:?}", syntax_result);
    let Ok(semantic_result) = Hook::parse(syntax_result, &HookParams::default()) else {
        return None;
    };
    // println!("semantic: {:?}", semantic_result);

    Some(PlushieDef {
        edges: semantic_result.edges.into(),
        nodes: semantic_result
            .colors
            .iter()
            .map(|c| Node {
                color: [c.0 as u8, c.1 as u8, c.2 as u8],
            })
            .collect(),
    })
}
