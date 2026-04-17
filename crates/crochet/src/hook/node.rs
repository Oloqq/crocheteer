use crate::{ColorRgb, acl::Origin, hook::Hook};

pub type NodeIndex = usize;

#[derive(Clone, Debug)]
pub struct Node {
    pub color: ColorRgb,
    pub peculiarity: Option<Peculiarity>,
    /// The location in the pattern that caused creation of this node.
    pub origin: Option<Origin>,
    /// Anchor of this node. Used for single loop forces.
    pub parent: Option<NodeIndex>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Peculiarity {
    Locked,
    Tip,
    /// Back-loop-only
    BLO(PointsOnPushPlane),
    /// Front-loop-only
    FLO(PointsOnPushPlane),
}

pub type PointsOnPushPlane = (usize, usize, usize);

pub struct NodeBuilder<'n> {
    node: &'n mut Node,
}

impl<'n> NodeBuilder<'n> {
    pub fn peculiarity(self, peculiarity: Peculiarity) -> Self {
        self.node.peculiarity = Some(peculiarity);
        self
    }

    pub fn peculiarity_opt(self, peculiarity: Option<Peculiarity>) -> Self {
        self.node.peculiarity = peculiarity;
        self
    }

    pub fn parent(self, parent: NodeIndex) -> Self {
        self.node.parent = Some(parent);
        self
    }

    pub fn parent_opt(self, parent: Option<NodeIndex>) -> Self {
        self.node.parent = parent;
        self
    }
}

impl Hook {
    pub fn add_node<'n>(&'n mut self, origin: Option<Origin>) -> NodeBuilder<'n> {
        self.nodes.push(Node {
            color: self.color,
            origin,
            peculiarity: None,
            parent: None,
        });
        NodeBuilder {
            node: self.nodes.last_mut().unwrap(),
        }
    }
}
