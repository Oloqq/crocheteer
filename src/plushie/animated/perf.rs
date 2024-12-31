use serde_derive::Serialize;
use std::time::Duration;

#[derive(Clone, Serialize, Default)]
pub struct Iteration {
    pub stuffing: Duration,
    pub skeletonization: Duration,
    pub normals: Duration,
    pub initial_cross: Duration,
    pub growing: Duration,
    pub part_selection: Duration,
}

impl Iteration {
    pub fn zeros() -> Self {
        Default::default()
    }
}
