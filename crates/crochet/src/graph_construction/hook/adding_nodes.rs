use super::Hook;

use crate::{
    acl::Origin,
    data::{Node, NodeIndex, Peculiarity},
};

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
    pub(super) fn add_node<'n>(&'n mut self, origin: Option<Origin>) -> NodeBuilder<'n> {
        self.nodes.push(Node {
            color: self.color,
            origin,
            peculiarity: None,
            parent: None,
        });
        self.edges.grow(); // prepare place for the next node
        NodeBuilder {
            node: self.nodes.last_mut().unwrap(),
        }
    }
}
