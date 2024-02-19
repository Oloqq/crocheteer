use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Stitch {
    Sc,
    Inc,
    Dec,
}

impl Display for Stitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp = format!("{:?}", self).to_lowercase();
        f.write_str(tmp.as_str())
    }
}

impl Stitch {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "sc" => Self::Sc.into(),
            "inc" => Self::Inc.into(),
            "dec" => Self::Dec.into(),
            _ => None,
        }
    }

    pub fn produced(&self) -> usize {
        use Stitch::*;
        match self {
            Inc => 2,
            Sc | Dec => 1,
        }
    }

    pub fn consumed(&self) -> usize {
        use Stitch::*;
        match self {
            Dec => 2,
            Sc | Inc => 1,
        }
    }
}

pub fn count_anchors_produced(round: &Vec<Stitch>) -> usize {
    round.iter().fold(0, |acc, stitch| acc + stitch.produced())
}

pub fn count_anchors_consumed(round: &Vec<Stitch>) -> usize {
    round.iter().fold(0, |acc, stitch| acc + stitch.consumed())
}
