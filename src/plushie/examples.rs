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
