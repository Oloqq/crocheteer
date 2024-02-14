use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Stitch {
    Sc,
    Inc,
    Dec,
}
