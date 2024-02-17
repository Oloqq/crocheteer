#![allow(unused)]

use crate::pattern::construction::PatternBuilder;
use crate::pattern::stitches::Stitch;
use Stitch::*;

use super::Plushie;

pub fn pillar() -> Plushie {
    let pattern = PatternBuilder::new(6).full_rounds(4).build().unwrap();
    Plushie::from_pattern(pattern)
}

pub fn bigball() -> Plushie {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .round_like(&vec![Sc, Inc])
        .full_rounds(1)
        .round_like(&vec![Sc, Dec])
        .round_like(&vec![Dec])
        .build()
        .unwrap();
    Plushie::from_pattern(pattern)
}

pub fn ball() -> Plushie {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .full_rounds(1)
        .round_like(&vec![Dec])
        .build()
        .unwrap();
    Plushie::from_pattern(pattern)
}
