pub mod force_graph;

mod acl;
mod errors;
mod hook;
#[allow(unused)]
mod params;
mod plushie_definition;

pub use crate::hook::hook_result::InitialGraph;
use crate::{
    acl::{PatternBuilder, PatternError},
    errors::Error,
    hook::{Hook, HookError, HookParams},
};
pub use hook::node::{Node, Peculiarity, PointsOnPushPlane};
pub use plushie_definition::*;

pub fn parse(acl_source: &str) -> Result<PlushieDef, Error> {
    let pattern = PatternBuilder::parse(acl_source).or_else(|e| Err(Error::Pattern(e)))?;
    let graph = Hook::parse(pattern, HookParams::default()).or_else(|e| Err(Error::Hook(e)))?;

    Ok(PlushieDef {
        edges: graph.edges.into(),
        nodes: graph.nodes,
    })
}

// TODO search for allow(unused)
