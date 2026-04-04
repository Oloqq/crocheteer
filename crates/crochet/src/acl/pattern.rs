use std::collections::HashMap;

#[derive(Debug)]
pub struct Pattern {
    #[allow(dead_code)] // TODO
    pub parameters: HashMap<String, String>,
    pub actions: Vec<Action>,
    pub cursor: usize,
}

impl Flow for Pattern {
    fn next(&mut self) -> Option<Action> {
        if self.cursor < self.actions.len() {
            let got = self.actions[self.cursor].clone();
            self.cursor += 1;
            Some(got)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<Action> {
        if self.cursor < self.actions.len() {
            let got = self.actions[self.cursor].clone();
            Some(got)
        } else {
            None
        }
    }
}

use crate::{ColorRgb, acl::Flow};

pub type Label = String;

// pub struct Action {
//     kind: ActionKind,
//     /// Byte location in the input string that produced this action
//     origin: (usize, usize),
// }

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
    Color(ColorRgb),
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
