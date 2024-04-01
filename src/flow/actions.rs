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

impl Action {
    pub fn parse(x: &str) -> Option<Self> {
        use Action::*;

        Some(match x {
            "sc" => Sc,
            "inc," => Inc,
            "dec," => Dec,
            // "ch(usize)," => Ch,
            // "attach(Label)," => Attach,
            // "reverse," => Reverse,
            // "flo," => FLO,
            // "blo," => BLO,
            // "bl," => BL,
            // "goto(Label)," => Goto,
            // "mark(Label)," => Mark,
            // "mr(usize)," => MR,
            // "fo," => FO,
            // "color(Color)," => Color,
            _ => return None,
        })
    }
}
