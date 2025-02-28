use std::{collections::HashMap, error::Error};

use serde_derive::{Deserialize, Serialize};

pub use super::animated::Leniency;

fn just_true() -> bool {
    true
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Params {
    /// Multiplier to all forces in a single step
    pub timestep: f32,
    /// Set to true if creation is meant to stand on it's own to simulate a flat bottom
    /// Set to false if the creation is carried around, so that the bottom is not flat
    #[serde(skip, default = "just_true")]
    pub floor: bool,
    /// Force that pulls the nodes down
    #[serde(skip)]
    pub gravity: f32,
    /// Distance between nodes that is considered "relaxed"
    pub desired_stitch_distance: f32,
    /// Configuration of centroid stuffing
    pub centroids: CentroidParams,
    /// Configuration of automatic simulation stopping
    #[serde(skip)]
    pub autostop: AutoStoppingParams,
    /// if true, the whole shape will be translated by displacement of root, so that root stays at (0, 0, 0).
    /// not applicable to LegacyPlushie
    #[serde(skip, default = "just_true")]
    pub reflect_locked: bool,
    /// Multipler for BLO/FLO force. If BLO/FLO behaves incorrectly, probably the sign is wrong.
    /// I assume it has to do with working the plushie clockwise vs counterclockwise.
    /// It has yet to be investigated.
    pub single_loop_force: f32,
    /// Method for setting initial positions of stitches
    pub initializer: Initializer,
    #[serde(skip)]
    pub hook_leniency: crate::plushie::animated::Leniency,
    /// Required displacement on a node for it to be affected. (Displacements with maginute below the threshold will be ignored)
    pub minimum_displacement: f32,

    /// Experimental multipart support
    pub multipart: bool,
    pub nodes: HashMap<String, LimbParam>,

    pub skelet_stuffing: SkeletParams,
    pub track_performance: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LimbParam {
    pub lock_x: Option<f32>,
    pub lock_y: Option<f32>,
    pub lock_z: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AutoStoppingParams {
    /// Minimal tension at which the Plushie is considered relaxed
    pub acceptable_tension: f32,
    /// Hard limit on the relaxing process
    pub max_relaxing_iterations: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CentroidParams {
    /// Number of centroids that simulate the stuffing. More centroids = more internal pressure. Bigger shapes need more.
    pub number: usize,
    pub force: f32,
    pub min_nodes_per_centroid: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SkeletParams {
    pub enable: bool,
    pub cluster_number: usize,
    pub must_include_points: f32,
    pub allowed_overlap: f32,
    pub autoskelet: bool,
    pub interval: usize,
    #[serde(skip)]
    pub interval_left: usize,
    #[serde(skip)]
    pub bones: Vec<crate::common::Point>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Initializer {
    /// Start with a few stitches, and build the plushie while simulation is running.
    OneByOne(OneByOneParams),
    /// Start with points arranged roughly in the shape of a cylinder
    Cylinder,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct OneByOneParams {
    /// Plushie will wait with expansion until the previous node is stabilized.
    /// Parameter sets the maximum displacement where the next node shall be added.
    pub acceptable_displacement_for_expanding: f32,
    /// If previous node cannot be stabilized, next one shall be added after set time.
    pub force_expansion_after_time: f32,
}

#[derive(Debug)]
pub enum ParamsError {
    Unknown(String),
}

impl std::fmt::Display for ParamsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ParamsError {}

impl Params {
    pub fn unconstrained_floating() -> Self {
        Self {
            timestep: 1.0,
            autostop: Default::default(),
            centroids: Default::default(),
            floor: false,
            gravity: 0.0,
            desired_stitch_distance: 1.0,
            reflect_locked: false,
            single_loop_force: 0.05,
            initializer: Initializer::Cylinder,
            minimum_displacement: 0.001,
            hook_leniency: Leniency::NoMercy,
            skelet_stuffing: Default::default(),
            track_performance: false,
            multipart: false,
            nodes: HashMap::new(),
        }
    }

    pub fn floored() -> Self {
        Self {
            floor: true,
            reflect_locked: true,
            ..Self::unconstrained_floating()
        }
    }

    fn update_one(&mut self, key: &str, val: &str) -> Result<(), Box<dyn Error>> {
        match key {
            "dsd" => self.desired_stitch_distance = val.parse()?,
            "centroids" => self.centroids.number = val.parse()?,
            "stuffing_force" => self.centroids.force = val.parse()?,
            "points_per_centroid" => self.centroids.min_nodes_per_centroid = val.parse()?,
            "single_loop_force" => self.single_loop_force = val.parse()?,
            "rooted" | "reflect_locked" => self.reflect_locked = val.parse()?,
            "floored" => self.floor = val.parse()?,
            "initializer" => {
                self.initializer = match val {
                    "cylinder" => Initializer::Cylinder,
                    "obo" | "onebyone" => Initializer::OneByOne(OneByOneParams::default()),
                    _ => {
                        log::debug!("Unknown value ({}) for parameter: {}", val, key);
                        Initializer::Cylinder
                    }
                }
            }
            "skelet_interval" => {
                self.skelet_stuffing.enable = true;
                self.skelet_stuffing.autoskelet = true;
                self.skelet_stuffing.interval = val.parse()?
            }
            "skelet_clusters" => self.skelet_stuffing.cluster_number = val.parse()?,
            "skelet_k1" => self.skelet_stuffing.must_include_points = val.parse()?,
            "skelet_k2" => self.skelet_stuffing.allowed_overlap = val.parse()?,
            "multipart" => self.multipart = val.parse()?,
            _ => {
                log::debug!("Unknown parameter: {}", key);
                return Err(Box::new(ParamsError::Unknown(key.to_owned())));
            }
        }
        return Ok(());
    }

    pub fn update(&mut self, src: &HashMap<String, String>) -> Vec<String> {
        let mut wrong = vec![];
        for (key, val) in src {
            match self.update_one(key, val) {
                Ok(_) => (),
                Err(err) => {
                    if err.is::<ParamsError>() {
                        match err.downcast::<ParamsError>().unwrap().as_ref() {
                            ParamsError::Unknown(unk) => {
                                wrong.push(format!("Unknown parameter: \"{}\"", unk))
                            }
                        }
                    }
                    wrong.push(format!("Bad value for parameter \"{}\" ({})", key, val))
                }
            }
        }
        wrong
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::floored()
    }
}

impl Default for CentroidParams {
    fn default() -> Self {
        Self {
            number: 2,
            force: 0.05,
            min_nodes_per_centroid: 60,
        }
    }
}

impl Default for SkeletParams {
    fn default() -> Self {
        Self {
            enable: false,
            cluster_number: 50,
            must_include_points: 0.95,
            allowed_overlap: 5.0,
            bones: vec![],
            autoskelet: true,
            interval: 50,
            interval_left: 0,
        }
    }
}

impl Default for OneByOneParams {
    fn default() -> Self {
        Self {
            acceptable_displacement_for_expanding: 0.03,
            force_expansion_after_time: 100.0,
        }
    }
}

impl Default for AutoStoppingParams {
    fn default() -> Self {
        Self {
            acceptable_tension: 0.02,
            max_relaxing_iterations: 100,
        }
    }
}

#[allow(unused)]
pub mod handpicked {
    use super::*;

    macro_rules! generate_get_handpicked {
        ($($name:ident),*) => {
            pub fn get(name: &str) -> Option<Params> {
                match name {
                    $(stringify!($name) => Some($name()),)*
                    _ => None,
                }
            }
        };
    }

    pub fn default() -> Params {
        Params::default()
    }

    pub fn grzib() -> Params {
        Params {
            autostop: AutoStoppingParams {
                // relaxes in 172 iterations
                acceptable_tension: 0.1,
                max_relaxing_iterations: 300,
            },
            centroids: CentroidParams {
                force: 0.2,
                number: 3,
                ..Default::default()
            },
            ..Params::floored()
        }
    }

    pub fn grzob() -> Params {
        Params {
            autostop: AutoStoppingParams {
                // relaxes in 172 iterations
                acceptable_tension: 0.1,
                max_relaxing_iterations: 300,
            },
            gravity: 0.0,
            single_loop_force: 0.0,
            centroids: CentroidParams {
                force: 0.2,
                number: 3,
                ..Default::default()
            },
            ..Params::floored()
        }
    }

    pub fn pillar() -> Params {
        Params {
            autostop: AutoStoppingParams {
                acceptable_tension: 0.000000002,
                max_relaxing_iterations: 500,
            },
            gravity: 0.0,
            single_loop_force: 0.0,
            centroids: CentroidParams {
                force: 0.05,
                number: 2,
                ..Default::default()
            },
            ..Params::floored()
        }
    }

    pub fn disk() -> Params {
        Params {
            autostop: AutoStoppingParams {
                acceptable_tension: 0.000000002,
                max_relaxing_iterations: 500,
            },
            gravity: 0.0,
            single_loop_force: 0.0,
            centroids: CentroidParams {
                force: 0.05,
                number: 3,
                ..Default::default()
            },
            ..Params::floored()
        }
    }

    generate_get_handpicked!(default, grzib, grzob, pillar, disk);
}
