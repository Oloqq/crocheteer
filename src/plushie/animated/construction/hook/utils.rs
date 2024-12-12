pub use crate::acl::actions::{Action, Label};
pub use crate::common::*;
pub use crate::plushie::animated::construction::Peculiarity;
pub use std::collections::VecDeque as Queue;

pub use Action::*;

#[derive(Debug)]
pub enum HookError {
    Empty,
    BadStarter,
    StarterInTheMiddle,
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
    /// Attach is supported only directly after a chain
    AttachWithoutChain,
    /// Simulation can't handle a node with too many links
    TooManyAnchorsForFO,
}

impl From<HookError> for String {
    fn from(value: HookError) -> Self {
        format!("{value:?}")
    }
}

#[derive(Clone, Debug)]
pub enum WorkingLoops {
    Both,
    Back,
    Front,
}
