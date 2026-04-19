use std::fmt::Display;

pub use crate::acl::Label;
use crate::acl::Origin;

#[derive(Debug)]
pub struct Error {
    pub code: ErrorCode,
    pub origin: Option<Origin>,
}

// TODO many of those should be unreachable given correct pattern parser (BadStarter, AnonymousMrInTheMiddle, DuplicateLabel, UnknownLabel)
#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    Internal(String),
    Empty,
    BadStarter,
    AnonymousMrInTheMiddle,
    DuplicateLabel(Label),
    UnknownLabel(Label),
    /// Tried to mark at a place where no anchors are available
    UselessMark,
    NoAnchorToPullThrough,
    FORequires2Anchors,
    SingleLoopOnNonAnchored,
    SingleLoopNoGrandparent,
    ChainOfZero,
    /// Chains are finished with some custom logic, chains one after another are currently not supported
    ChainAfterChain,
    /// Simulation can't handle a node with too many links
    TooManyAnchorsForFO,
    /// Annotation says the user expected a different number of available anchors at this point
    WrongAnnotation {
        expected: usize,
        actual: usize,
        location: (usize, usize),
    },
    // TODO this variant was used for "arounds". It should also be used with regular repetitions. With any repetition, marks and gotos make no sense.
    IllegalActionInRepetition,
}

impl ErrorCode {
    /// Some situations should be prevented by ACL parser
    pub fn means_bug_in_crate(&self) -> bool {
        use ErrorCode::*;
        match self {
            Internal(_) => true,
            BadStarter => true,
            DuplicateLabel(_) => true,
            UnknownLabel(_) => true,
            // ---
            Empty => false,
            AnonymousMrInTheMiddle => false,
            UselessMark => false,
            NoAnchorToPullThrough => false,
            FORequires2Anchors => false,
            SingleLoopOnNonAnchored => false,
            SingleLoopNoGrandparent => false,
            ChainOfZero => false,
            ChainAfterChain => false,
            TooManyAnchorsForFO => false,
            WrongAnnotation { .. } => false,
            IllegalActionInRepetition => false,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin_msg = match self.origin {
            Some(origin) => format!(
                "at executing action that originated at bytes {}..{}",
                origin.as_range().start,
                origin.as_range().end
            ),
            None => format!("without specified origin"),
        };

        write!(
            f,
            "{:?} {origin_msg}", // to display the actual text, or line pos, formatter need to analyze the input string
            self.code,
        )
    }
}
