use std::{collections::HashMap, ops::Range};

use pest::Span;

use crate::{ColorRgb, acl::Flow};

#[derive(Debug, Clone)]
pub struct PatternAst {
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone)]
pub struct Part {
    pub name: String,
    pub instances: usize,
    pub actions: Vec<ActionWithOrigin>,
    pub parameters: PartParameters,
}

#[derive(Debug, Clone, Default)]
pub struct PartParameters {
    pub centroids: usize,
    pub other: HashMap<String, String>,
}

pub struct PatternIter<'p> {
    pub pattern: &'p PatternAst,
    pub action_cursor: usize,
    pub part_cursor: usize,
}

impl PatternAst {
    pub fn as_iter<'p>(&'p self) -> PatternIter<'p> {
        PatternIter {
            pattern: &self,
            action_cursor: 0,
            part_cursor: 0,
        }
    }
}

impl<'p> Flow for PatternIter<'p> {
    fn next(&mut self) -> Option<Action> {
        self.next_with_origin()
            .map(|action_with_origin| action_with_origin.action)
    }

    fn peek(&self) -> Option<Action> {
        self.peek_with_origin()
            .map(|action_with_origin| action_with_origin.action)
    }

    fn next_with_origin(&mut self) -> Option<ActionWithOrigin> {
        if self.part_cursor < self.pattern.parts.len() {
            if self.action_cursor < self.pattern.parts[self.part_cursor].actions.len() {
                let got = self.pattern.parts[self.part_cursor].actions[self.action_cursor].clone();
                self.action_cursor += 1;
                Some(got)
            } else {
                self.part_cursor += 1;
                self.action_cursor = 0;
                self.next_with_origin()
            }
        } else {
            None
        }
    }

    fn peek_with_origin(&self) -> Option<ActionWithOrigin> {
        if self.part_cursor < self.pattern.parts.len() {
            if self.action_cursor < self.pattern.parts[self.part_cursor].actions.len() {
                let got = self.pattern.parts[self.part_cursor].actions[self.action_cursor].clone();
                Some(got)
            } else if self.part_cursor + 1 < self.pattern.parts.len() {
                let got = self.pattern.parts[self.part_cursor + 1].actions[0].clone();
                Some(got)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub type Label = String;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Origin {
    // can't store positions as range because ops::Range<_> does not implement Copy because it is an iterator: https://github.com/rust-lang/rust/pull/27186
    byte_start: usize,
    byte_end: usize,
}

impl Origin {
    pub fn from_start_end(start: usize, end: usize) -> Self {
        Self {
            byte_start: start,
            byte_end: end,
        }
    }

    pub fn from_span(span: Span) -> Self {
        Self {
            byte_start: span.start(),
            byte_end: span.end(),
        }
    }

    pub fn as_range(&self) -> Range<usize> {
        self.byte_start..self.byte_end
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionWithOrigin {
    pub action: Action,
    /// Location in the input string that produced this action.
    pub origin: Option<Origin>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Single Crochet
    Sc,
    /// Increase
    Inc,
    /// Decrease
    Dec,
    /// Slip stitch. Does not create an anchor.
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
    BeginPart,
    EndPart,
}

impl Action {
    pub(crate) fn with_origin(self, span: Span) -> ActionWithOrigin {
        ActionWithOrigin {
            action: self,
            origin: Some(Origin::from_span(span)),
        }
    }

    pub(crate) fn without_origin(self) -> ActionWithOrigin {
        ActionWithOrigin {
            action: self,
            origin: None,
        }
    }

    pub(crate) fn is_repeatable(&self) -> bool {
        use Action::*;
        match &self {
            Sc | Inc | Dec | Slst => true,
            Attach(_, _)
            | FLO
            | BLO
            | BL
            | Goto(_)
            | Mark(_)
            | MR(_)
            | FO
            | Color(_)
            | Sew(_, _)
            | EnforceAnchors(_, _)
            | BeginPart
            | EndPart => false,
        }
    }
}
