pub mod force_graph;

mod acl;
mod errors;
mod hook;
mod plushie_definition;

pub use crate::hook::hook_result::InitialGraph;
pub use acl::{Origin, PatternAst};
pub use hook::node::{Node, Peculiarity, PointsOnPushPlane};
pub use plushie_definition::*;

use crate::{
    acl::{PatternBuilder, PatternError},
    errors::Error,
};
use hook::{Hook, HookParams};

pub fn parse(acl_source: &str) -> Result<PlushieDef, Error> {
    let pattern = PatternBuilder::parse(acl_source).or_else(|e| Err(Error::Pattern(e)))?;
    let hook_params = HookParams {
        tip_from_fo: true,
        enforce_counts: false,
    };
    let graph = Hook::parse(pattern.as_iter(), hook_params).or_else(|e| Err(Error::Hook(e)))?;
    assert!(graph.nodes.len() == graph.edges.len());

    Ok(PlushieDef {
        pattern,
        edges: graph.edges.into(),
        nodes: graph.nodes,
    })
}

#[cfg(test)]
mod tests;
