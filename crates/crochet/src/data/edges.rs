/// Edges guarantee that the underlying 2D vector that represents the yarn
/// satisfies the condition that:
/// for every `i` each element of `edges[i]` is smaller than `i`
#[derive(Debug, Clone)]
pub struct Edges {
    edges: Vec<Vec<usize>>,
}

impl Edges {
    pub fn new() -> Self {
        Self {
            // for graph building, element in edges must exist before its corresponding node is registered
            edges: vec![Vec::with_capacity(6)],
        }
    }

    pub fn from(mby_unordered: Vec<Vec<usize>>) -> Self {
        let mut res = Self {
            edges: vec![vec![]; mby_unordered.len()],
        };
        for (node1, destinations) in mby_unordered.into_iter().enumerate() {
            for node2 in destinations {
                res.link(node1, node2);
            }
        }
        res
    }

    pub fn link(&mut self, node1: usize, node2: usize) {
        assert!(node1 != node2, "Node can't link to itself");
        assert!(
            node1 < self.edges.len() && node2 < self.edges.len(),
            "Both nodes to link should already have their spots in Edges ({}, {}, len: {})",
            node1,
            node2,
            self.edges.len()
        );
        if node2 > node1 {
            self.edges[node2].push(node1);
        } else {
            self.edges[node1].push(node2);
        }
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn grow(&mut self) {
        self.edges.push(Vec::with_capacity(2)); // 2 because Sc (the most common stitch) will link itself to 2 nodes
    }

    pub fn cleanup(&mut self) {
        assert!(self.edges.last().unwrap().len() == 0);
        self.edges.pop();
    }

    pub fn last(&self) -> Option<&Vec<usize>> {
        self.edges.last()
    }

    pub fn data(&self) -> &Vec<Vec<usize>> {
        &self.edges
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<usize>> {
        self.edges.iter()
    }

    pub fn edges_from_node(&self, i: usize) -> &Vec<usize> {
        &self.edges[i]
    }

    pub fn from_trimmed(source: Self, cutoff: usize) -> Self {
        Self {
            edges: source.edges.into_iter().take(cutoff).collect(),
        }
    }

    pub(crate) fn clone_next_node(&mut self, other: &Self) {
        self.edges.push(other.edges[self.edges.len()].clone());
    }
}

impl Into<Vec<Vec<usize>>> for Edges {
    fn into(self) -> Vec<Vec<usize>> {
        self.edges
    }
}

impl PartialEq for Edges {
    fn eq(&self, other: &Self) -> bool {
        use std::collections::HashSet;

        if self.edges.len() != other.edges.len() {
            return false;
        }

        for (my, their) in self.edges.iter().zip(&other.edges) {
            let myset: HashSet<usize> = HashSet::from_iter(my.iter().cloned());
            let theirset: HashSet<usize> = HashSet::from_iter(their.iter().cloned());
            if myset != theirset {
                return false;
            }
        }

        return true;
    }
}

#[derive(Clone, Debug)]
pub struct DeferredEdge {
    pub with_node: usize,
    pub node_a: usize,
    pub node_b: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edges_from_reorders_indexes() {
        let src = vec![
            vec![1, 2, 3], // 0
            vec![2, 4],    // 1
            vec![3, 5],    // 2
            vec![4, 6],    // 3
            vec![5],       // 4
            vec![6],       // 5
            vec![],        // 6
        ];
        let e = Edges::from(src);
        assert_eq!(
            e.edges,
            vec![
                vec![],     // 0
                vec![0],    // 1
                vec![0, 1], // 2
                vec![0, 2], // 3
                vec![1, 3], // 4
                vec![2, 4], // 5
                vec![3, 5], // 6
            ]
        )
    }
}
