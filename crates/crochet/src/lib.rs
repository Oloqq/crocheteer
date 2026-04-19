pub mod data;
pub mod force_graph;

pub mod acl;
pub mod errors;
mod graph_construction;
mod plushie_definition;

pub use plushie_definition::*;

use crate::{
    acl::PatternBuilder, errors::Error, force_graph::Initializer,
    force_graph::simulated_plushie::SimulatedPlushie,
};
use graph_construction::HookParams;

pub fn parse(
    acl_source: &str,
    hook_size: f32,
    initializer: &Initializer,
) -> Result<(PlushieDef, SimulatedPlushie), Error> {
    let pattern = PatternBuilder::parse(acl_source).or_else(|e| Err(Error::Pattern(e)))?;
    let hook_params = HookParams {
        tip_from_fo: true,
        enforce_counts: false,
    };
    let graph = graph_construction::parse(pattern.as_iter(), hook_params)
        .or_else(|e| Err(Error::Hook(e)))?;
    assert!(graph.nodes.len() == graph.edges.len());

    let definition = PlushieDef {
        pattern,
        edges: graph.edges,
        nodes: graph.nodes,
    };
    Ok((
        definition.clone(),
        SimulatedPlushie::from(definition, initializer, hook_size, &graph.part_limits),
    ))
}

#[cfg(test)]
mod tests;
