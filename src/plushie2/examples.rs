#![allow(unused)]

use crate::pattern::builder::PatternBuilder;
use crate::pattern::stitches::Stitch;
use crate::pattern::Pattern;
use Stitch::*;

use super::Plushie;

pub fn get(name: &str) -> Option<(Pattern, Plushie)> {
    Some(match name {
        _ => return None,
    })
}
