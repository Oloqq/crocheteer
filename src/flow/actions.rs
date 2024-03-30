use crate::common::Color;

pub type Label = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Sc,
    Inc,
    Dec,
    Ch(usize),
    /// Pull yarn through a spot
    Attach(Label),
    /// Begin working in the other direction
    Reverse,
    /// Front loop only
    FLO,
    /// Back loop only
    BLO,
    /// Both loops
    BL,
    /// Let go of the yarn, start working elsewhere
    Goto(Label),
    /// Mark a spot that will be important later
    Mark(Label),
    /// Magic ring
    MR(usize),
    /// Fasten off
    FO,
    /// Change yarn color
    Color(Color),
}
