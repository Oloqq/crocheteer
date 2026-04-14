pub mod force_graph;

mod acl;
mod errors;
mod hook;
mod plushie_definition;

pub use crate::hook::hook_result::InitialGraph;
use crate::{acl::PatternError, errors::Error, hook::HookError};
pub use acl::{Origin, Pattern, PatternBuilder};
pub use hook::{
    node::{Node, Peculiarity, PointsOnPushPlane},
    {Hook, HookParams},
};
pub use plushie_definition::*;

pub fn acl_to_pattern(acl_source: &str) -> Result<Pattern, acl::PatternError> {
    PatternBuilder::parse(acl_source)
}

pub fn parse(acl_source: &str) -> Result<PlushieDef, Error> {
    let pattern = PatternBuilder::parse(acl_source).or_else(|e| Err(Error::Pattern(e)))?;
    let graph = Hook::parse(pattern, HookParams::default()).or_else(|e| Err(Error::Hook(e)))?;

    Ok(PlushieDef {
        edges: graph.edges.into(),
        nodes: graph.nodes,
    })
}
