mod acl;
mod force_graph;
mod hook;
#[allow(unused)] // TODO unused
mod params;
mod plushie_definition;

pub use crate::hook::hook_result::InitialGraph;
pub use force_graph::{
    centroid_push_magnitude, centroid_stuffing, initializers::Initializer, link_force_magnitude,
    link_forces, weight,
};
pub use plushie_definition::*;

use crate::{
    acl::{PatternBuilder, PatternError},
    hook::{Hook, HookError, HookParams},
};
use std::fmt::Display;

pub type ColorRgb = [u8; 3];

#[derive(Debug)]
pub enum Error {
    Pattern(PatternError),
    Hook(HookError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Pattern(e) => write!(f, "pattern error: {e}"),
            Error::Hook(e) => write!(f, "hook error: {e:?}"),
        }
    }
}

pub fn parse(acl_source: &str) -> Result<PlushieDef, Error> {
    let pattern = PatternBuilder::parse(acl_source).or_else(|e| Err(Error::Pattern(e)))?;
    let graph = Hook::parse(pattern, &HookParams::default()).or_else(|e| Err(Error::Hook(e)))?;

    Ok(PlushieDef {
        edges: graph.edges.into(),
        nodes: graph
            .colors
            .iter()
            .map(|color| Node { color: *color })
            .collect(),
    })
}
