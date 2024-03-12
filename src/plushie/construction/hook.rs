use crate::common::*;
use crate::{flow::actions::Action, plushie::Edges};
use HookError::*;

pub struct HookResult {
    pub edges: Edges,
    pub positions: Vec<Point>,
    pub approximate_height: f32,
}

#[derive(Debug)]
pub enum HookError {
    BadStarter,
}

impl From<HookError> for String {
    fn from(value: HookError) -> Self {
        format!("{value:?}")
    }
}

// chains shall be approximated as a line from start point to attachment point
// how to avoid mutiple shoves of the nodes array during construction (e.g. with multiple FOs that should be placed at the beginning)

/// Responsible for building the graph used in the simulation
pub struct Hook {
    edges: Edges,
    nodes: Vec<Point>,
}

impl Hook {
    pub fn start_with(action: &Action) -> Result<Self, HookError> {
        if !action.is_starter() {
            return Err(BadStarter);
        }

        Ok(Self {
            edges: vec![],
            nodes: vec![],
        })
    }

    pub fn finish(self) -> HookResult {
        HookResult {
            edges: self.edges,
            positions: self.nodes,
            approximate_height: 3.0,
        }
    }

    pub fn perform(&mut self, _action: &Action) {}
}
