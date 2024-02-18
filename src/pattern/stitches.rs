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
}

pub fn count_anchors_produced(round: &Vec<Stitch>) -> usize {
    let mut result = 0;
    for stitch in round {
        use Stitch::*;
        result += match stitch {
            Inc => 2,
            Sc | Dec => 1,
        }
    }
    result
}

pub fn count_anchors_consumed(round: &Vec<Stitch>) -> usize {
    let mut result = 0;
    for stitch in round {
        use Stitch::*;
        result += match stitch {
            Dec => 2,
            Sc | Inc => 1,
        }
    }
    result
}
