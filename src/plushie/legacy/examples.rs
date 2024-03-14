#![allow(unused)]

use crate::pattern::builder::PatternBuilder;
use crate::pattern::stitches::Stitch;
use crate::pattern::Pattern;
use Stitch::*;

use super::Plushie;

pub fn get(name: &str) -> Option<(Pattern, Plushie)> {
    Some(match name {
        "pillar" => pillar(),
        "bigball" => bigball(),
        "ball" => ball(),
        "bigpillar" => bigpillar(),
        "vase" => vase(),
        "bowl" => bowl(),
        _ => return None,
    })
}

pub fn pillar() -> (Pattern, Plushie) {
    let pattern = PatternBuilder::new(6).full_rounds(4).fasten_off().unwrap();
    let plushie = Plushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn bigball() -> (Pattern, Plushie) {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .round_like(&vec![Sc, Inc])
        .full_rounds(1)
        .round_like(&vec![Sc, Dec])
        .round_like(&vec![Dec])
        .fasten_off()
        .unwrap();
    let plushie = Plushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn ball() -> (Pattern, Plushie) {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .full_rounds(1)
        .round_like(&vec![Dec])
        .fasten_off()
        .unwrap();
    let plushie = Plushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn bigpillar() -> (Pattern, Plushie) {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .full_rounds(6)
        .round_like(&vec![Dec])
        .fasten_off()
        .unwrap();
    let plushie = Plushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn vase() -> (Pattern, Plushie) {
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .full_rounds(6)
        .loose_end()
        .unwrap();
    let plushie = Plushie::from_pattern(&pattern);
    (pattern, plushie)
}

pub fn bowl() -> (Pattern, Plushie) {
    let pattern = Pattern::from_human_readable(
        "@centroids = 6
    @floor = true
    : MR 6 (6)
    : 6 inc (12)
    : [inc, sc] x 6 (18)
    : [inc, 2 sc] x 6 (24)
    : [inc, 3 sc] x 6 (30)
    : [inc, 4 sc] x 6 (36)
    : 36 sc (36) # BLO
    : 36 sc (36)
    : [inc, 5 sc] x 6 (42)
    2: 42 sc (42)
    : [inc, 6 sc] x 6 (48)",
    )
    .unwrap();
    let plushie = Plushie::from_pattern(&pattern);
    (pattern, plushie)
}
