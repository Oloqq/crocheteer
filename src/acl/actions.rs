use crate::common::colors;

pub type Label = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Sc,
    Inc,
    Dec,
    /// Slip stitch
    Slst,
    Ch(usize), // FIXME supported?
    /// Create a chain, then attach it to a marked position
    Attach(Label, usize),
    /// Begin working in the other direction
    Reverse, // FIXME unupported
    /// Front loop only
    FLO,
    /// Back loop only
    BLO,
    /// Both loops
    BL,
    /// Let go of the yarn, start working elsewhere
    Goto(Label),
    /// Mark a spot that will be important later.
    /// Mark preceding an attach will point to the round that was left behind.
    Mark(Label),
    /// Magic ring
    MR(usize),
    ///
    MRConfigurable(usize, String),
    /// Fasten off
    FO,
    /// Change yarn color
    Color(colors::Color),
    /// Connect two stitches
    Sew(Label, Label),
    /// Verify the number of available anchors
    EnforceAnchors(usize, (usize, usize)),
}

impl Action {
    pub fn anchors_consumed(&self) -> u32 {
        use Action::*;
        match self {
            Sc | Inc | Slst => 1,
            Dec => 2,
            MR(_) | MRConfigurable(..) => 0,
            FO => 0, // FO in some way consumes the anchors, but it is handled in another way
            Ch(_) | Reverse => unimplemented!(),
            Attach(..) => 0,
            FLO | BLO | BL | Goto(_) | Mark(_) | Color(_) | EnforceAnchors(..) | Sew(..) => 0,
        }
    }

    pub fn anchors_produced(&self) -> u32 {
        use Action::*;
        match self {
            Sc | Dec | Slst => 1,
            Inc => 2,
            MR(x) | MRConfigurable(x, _) => *x as u32,
            FO => 0,
            Ch(_) | Reverse => unimplemented!(),
            Attach(_, chain_size) => *chain_size as u32,
            FLO | BLO | BL | Goto(_) | Mark(_) | Color(_) | EnforceAnchors(..) | Sew(..) => 0,
        }
    }
}
