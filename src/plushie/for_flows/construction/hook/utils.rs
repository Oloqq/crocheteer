pub use crate::common::*;
pub use crate::flow::actions::{Action, Label};
pub use crate::plushie::for_flows::construction::Peculiarity;

pub use Action::*;

#[derive(Debug)]
pub enum HookError {
    Empty,
    BadStarter,
    StarterInTheMiddle,
    ChainStart,
    TriedToWorkAfterFastenOff,
    DuplicateLabel(Label),
    UnknownLabel(Label),
    CantMarkAfterFO,
    NoAnchorToPullThrough,
    FORequires2Anchors,
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
