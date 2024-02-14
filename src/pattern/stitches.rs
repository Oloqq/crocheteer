use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Stitch {
    Sc,
    Inc,
    Dec,
}

pub fn count_anchors(round: &Vec<Stitch>) -> usize {
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
