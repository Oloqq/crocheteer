use pest::Span;

use crate::{ColorRgb, acl::Flow};

#[derive(Debug)]
pub struct Pattern {
    pub actions: Vec<ActionWithOrigin>,
    pub cursor: usize,
}

impl Flow for Pattern {
    fn next(&mut self) -> Option<Action> {
        self.next_with_origin()
            .map(|action_with_origin| action_with_origin.action)
    }

    fn peek(&self) -> Option<Action> {
        self.peek_with_origin()
            .map(|action_with_origin| action_with_origin.action)
    }

    fn next_with_origin(&mut self) -> Option<ActionWithOrigin> {
        if self.cursor < self.actions.len() {
            let got = self.actions[self.cursor].clone();
            self.cursor += 1;
            Some(got)
        } else {
            None
        }
    }

    fn peek_with_origin(&self) -> Option<ActionWithOrigin> {
        if self.cursor < self.actions.len() {
            let got = self.actions[self.cursor].clone();
            Some(got)
        } else {
            None
        }
    }
}

pub type Label = String;
pub type ByteRange = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub struct ActionWithOrigin {
    pub action: Action,
    /// Byte location in the input string that produced this action. Will be (0, 0) for implicit BL at round start.
    pub origin: ByteRange,
}

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
    /// Fasten off
    FO,
    /// Change yarn color
    Color(ColorRgb),
    /// Connect two stitches
    Sew(Label, Label),
    /// Verify the number of available anchors
    EnforceAnchors(usize, (usize, usize)),
}

impl Action {
    pub(crate) fn with_origin(self, span: Span) -> ActionWithOrigin {
        ActionWithOrigin {
            action: self,
            origin: (span.start(), span.end()),
        }
    }

    pub(crate) fn with_origin_range(self, byte_range: ByteRange) -> ActionWithOrigin {
        ActionWithOrigin {
            action: self,
            origin: byte_range,
        }
    }

    pub(crate) fn without_origin(self) -> ActionWithOrigin {
        ActionWithOrigin {
            action: self,
            origin: (0, 0),
        }
    }
}
