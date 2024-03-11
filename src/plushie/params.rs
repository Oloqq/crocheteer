use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Params {
    /// Number of centroids that simulate the stuffing. More centroids = more internal pressure. Bigger shapes need more
    pub centroids: usize,
    /// Set to true if creation is meant to stand on it's own to simulate a flat bottom
    /// Set to false if the creation is carried around, so that the bottom is not flat
    pub floor: bool,
    /// Force pulling the nodes down
    pub gravity: f32,
    /// Distance between nodes that is considered "relaxed"
    pub desired_stitch_distance: f32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            centroids: 2,
            floor: false,
            gravity: 5e-4,
            desired_stitch_distance: 1.0,
        }
    }
}
