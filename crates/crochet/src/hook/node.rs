use crate::{ColorRgb, acl::ByteRange, hook::Hook};

#[derive(Clone, Debug)]
pub struct Node {
    pub color: ColorRgb,
    pub peculiarity: Option<Peculiarity>,
    /// The bytes in the pattern that caused creation of this node.
    pub origin: ByteRange,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Peculiarity {
    Locked,
    Tip,
    BLO(PointsOnPushPlane),
    FLO(PointsOnPushPlane),
}

pub type PointsOnPushPlane = (usize, usize, usize);

impl Hook {
    pub fn add_node(&mut self, peculiarity: Option<Peculiarity>, origin: ByteRange) {
        self.nodes.push(Node {
            color: self.color,
            peculiarity,
            origin,
        });
    }
}
