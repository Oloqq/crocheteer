use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Params {
    /// Minimal tension at which the Plushie is considered relaxed
    pub acceptable_tension: f32,
    /// Hard limit on the relaxing process
    pub max_relaxing_iterations: usize,
    /// Set to true if creation is meant to stand on it's own to simulate a flat bottom
    /// Set to false if the creation is carried around, so that the bottom is not flat
    pub floor: bool,
    /// Force pulling the nodes down
    pub gravity: f32,
    /// Distance between nodes that is considered "relaxed"
    pub desired_stitch_distance: f32,
    /// Configuration of centroid stuffing
    pub centroids: CentroidParams,
    /// if true, the whole shape will be translated by displacement of root, so that root stays at (0, 0, 0).
    /// not applicable to LegacyPlushie
    pub keep_root_at_origin: bool,
    /// if true, points will not be allowed to go below root
    /// this plus gravity simulates the plushie sitting on a flat surface
    pub sitting: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CentroidParams {
    /// Number of centroids that simulate the stuffing. More centroids = more internal pressure. Bigger shapes need more.
    pub number: usize,
    pub force: f32,
}

impl Default for Params {
    fn default() -> Self {
        const THIS_DEFAULT_IS_TRASH: f32 = 0.02;
        Self {
            centroids: Default::default(),
            floor: false,
            gravity: 5e-4,
            desired_stitch_distance: 1.0,
            acceptable_tension: THIS_DEFAULT_IS_TRASH,
            max_relaxing_iterations: 100,
            keep_root_at_origin: true,
            sitting: true,
        }
    }
}

impl Default for CentroidParams {
    fn default() -> Self {
        Self {
            number: 2,
            force: 0.05,
        }
    }
}
