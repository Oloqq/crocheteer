use crate::{ColorRgb, acl::ActionWithOrigin};

pub type NodeIndex = usize;

#[derive(Clone, Debug)]
pub struct Node {
    pub color: ColorRgb,
    pub peculiarity: Option<Peculiarity>,
    /// The location in the pattern that caused creation of this node.
    pub origin: ActionWithOrigin,
    /// Anchor of this node. Used for single loop forces.
    pub(crate) parent: Option<NodeIndex>,
    pub part_index: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Peculiarity {
    Locked,
    /// Virtual node of Fasten-Off
    Tip,
    /// Back-loop-only
    BLO(PointsOnPushPlane),
    /// Front-loop-only
    FLO(PointsOnPushPlane),
}

pub type PointsOnPushPlane = (usize, usize, usize);
