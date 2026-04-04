pub use std::collections::VecDeque as Queue;

pub use crate::acl::{Action, Label};

// TODO many of those should be unreachable given correct pattern parser (BadStarter, AnonymousMrInTheMiddle, DuplicateLabel, UnknownLabel)
#[derive(Debug)]
pub enum HookError {
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

impl From<HookError> for String {
    fn from(value: HookError) -> Self {
        use HookError::*;
        match value {
            // Empty => todo!(),
            // BadStarter => todo!(),
            AnonymousMrInTheMiddle => "MR in the middle of the pattern
is only allowed with second argument being an identifier
and with @multipart=true
e.g. MR(6, second_part)"
                .into(),

            // DuplicateLabel(_) => todo!(),
            // UnknownLabel(_) => todo!(),
            // UselessMark => todo!(),
            // NoAnchorToPullThrough => todo!(),
            // FORequires2Anchors => todo!(),
            // SingleLoopOnNonAnchored => todo!(),
            // SingleLoopNoGrandparent => todo!(),
            // ChainOfZero => todo!(),
            // ChainAfterChain => todo!(),
            // TooManyAnchorsForFO => todo!(),
            _ => format!("{value:?}"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WorkingLoops {
    Both,
    Back,
    Front,
}
