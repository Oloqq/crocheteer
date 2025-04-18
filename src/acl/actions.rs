use crate::common::colors;

pub type Label = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Single Crochet
    Sc,
    /// Increase
    Inc,
    /// Decrease
    Dec,
    /// Slip stitch
    Slst,
    /// Create a chain, then attach it to a marked position
    Attach(Label, usize),
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
    /// Beginning of a sequence going all around the round
    AroundStart,
    /// End of a sequence going all around the round
    AroundEnd,
}

impl Action {
    pub fn is_physical_stitch(&self) -> bool {
        use Action::*;
        match self {
            Sc | Inc | Dec | Slst => true,
            _ => false,
        }
    }
}
