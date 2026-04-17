pub mod data;
pub mod force_graph;

mod acl;
mod errors;
mod graph_construction;
mod plushie_definition;

pub use acl::{Origin, PatternAst};
pub use plushie_definition::*;

use crate::{acl::PatternBuilder, errors::Error};
use graph_construction::HookParams;

pub fn parse(acl_source: &str) -> Result<PlushieDef, Error> {
    let pattern = PatternBuilder::parse(acl_source).or_else(|e| Err(Error::Pattern(e)))?;
    let hook_params = HookParams {
        tip_from_fo: true,
        enforce_counts: false,
    };
    let graph = graph_construction::parse(pattern.as_iter(), hook_params)
        .or_else(|e| Err(Error::Hook(e)))?;
    assert!(graph.nodes.len() == graph.edges.len());

    Ok(PlushieDef {
        pattern,
        edges: graph.edges.into(),
        nodes: graph.nodes,
    })
}

#[cfg(test)]
mod tests;
