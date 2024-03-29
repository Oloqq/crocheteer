use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub centroids: usize,
    pub floor: bool,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            centroids: 2,
            floor: false,
        }
    }
}
