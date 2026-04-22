use crate::data::{Edges, Node};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug)]
pub(crate) struct InitialGraph {
    pub edges: Edges,
    pub nodes: Vec<Node>,
    pub part_limits: Vec<usize>,
    #[allow(dead_code)]
    pub mark_to_node: HashMap<String, usize>,
    pub part_joins: PartClusters,
}

#[derive(Debug, Clone)]
pub struct PartClusters {
    part_to_cluster: Vec<usize>,
    joins: Vec<PartJoin>,
    cursor: usize,
}

impl PartClusters {
    pub fn new(parts: usize, joins: HashSet<PartJoin>) -> Self {
        for j in &joins {
            assert!(
                j.from < parts && j.to < parts,
                "invalid joins for part tree"
            );
        }
        let mut joins: Vec<PartJoin> = joins.into_iter().collect();
        joins.sort_by(|a, b| {
            a.with_node
                .cmp(&b.with_node)
                .then(a.ordering.cmp(&b.ordering))
        });

        Self {
            part_to_cluster: (0..parts).into_iter().map(|i| i).collect(),
            joins,
            cursor: 0,
        }
    }

    pub fn index_of_next_join(&self) -> Option<usize> {
        self.joins.get(self.cursor).map(|x| x.with_node)
    }

    pub fn perform_next_join(&mut self) {
        if self.cursor >= self.joins.len() {
            return;
        }
        let part1 = self.joins[self.cursor].from;
        let part2 = self.joins[self.cursor].to;
        self.cursor += 1;
        assert!(part1 < self.part_to_cluster.len() && part2 < self.part_to_cluster.len());

        let replaced_cluster = self.part_to_cluster[part1];
        let expanding_cluster = self.part_to_cluster[part2];

        for cluster in self.part_to_cluster.iter_mut() {
            if cluster == &replaced_cluster {
                *cluster = expanding_cluster;
            }
        }
    }

    pub fn perform_all_joins(&mut self) {
        for _ in 0..self.joins.len() {
            self.perform_next_join();
        }
    }

    pub fn get_part_cluster(&self, part: usize) -> usize {
        assert!(part < self.part_to_cluster.len());
        self.part_to_cluster[part]
    }
}

/// Represents the act of joining previously separate parts with a yarn.
/// Parts are identified by their index.
/// Joins are first registered in a set, they are only hashed on the participating parts.
#[derive(Clone, Debug)]
pub struct PartJoin {
    pub from: usize,
    pub to: usize,
    pub with_node: usize, // not hashed
    // additional ordering when sorting, necessary for deterministic result when multiple sews are used in succession
    pub ordering: u32, // not hashed
}

impl Hash for PartJoin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
        // omitted from hashing, only the first instance of with_node is important
        // with_node
        // ordering
    }
}

impl PartialEq for PartJoin {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
        // omitted from hashing
        // with_node
        // ordering
    }
}

impl std::cmp::Eq for PartJoin {}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::{
        acl::PatternBuilder,
        graph_construction::{self, HookParams},
    };

    use super::*;

    #[test]
    fn test_part_joins_with_different_nodes_share_hash() {
        let p1 = PartJoin {
            with_node: 4,
            from: 1,
            to: 0,
            ordering: 0,
        };

        let p2 = PartJoin {
            with_node: 8,
            from: 1,
            to: 0,
            ordering: 0,
        };

        let mut set: HashSet<PartJoin> = HashSet::new();
        set.insert(p1.clone());
        assert!(set.contains(&p2));
        set.insert(p2.clone());
        assert_eq!(set.get(&p1).unwrap().with_node, 4);
        assert_eq!(set.get(&p2).unwrap().with_node, 4);
    }

    #[test]
    fn test_part_joins() {
        let acl = indoc::indoc! {"
        == Part1 ==
        : MR(6)
        : 6 sc, mark(p1)
        FO

        == Part2 ==
        : MR(6)
        : 6 sc, mark(p2), mark(p3)
        FO

        sew(p1, p2)

        == Part3 ==
        : MR(6)
        : 6 sc, mark(p4)
        FO

        sew(p4, p3)
    "};
        let pattern = PatternBuilder::parse(acl).unwrap();
        let hook_params = HookParams {
            tip_from_fo: true,
            enforce_counts: false,
        };
        let graph = graph_construction::parse(pattern.as_iter(), hook_params).unwrap();
        assert_eq!(graph.part_joins.part_to_cluster.len(), 3);

        assert_eq!(graph.part_joins.joins.len(), 2);
        assert_eq!(graph.part_joins.joins[0].with_node, 28);
        assert_eq!(graph.part_joins.joins[0].from, 1);
        assert_eq!(graph.part_joins.joins[0].to, 0);

        assert_eq!(graph.part_joins.joins[1].with_node, 42);
        assert_eq!(graph.part_joins.joins[1].from, 2);
        assert_eq!(graph.part_joins.joins[1].to, 1);

        assert_eq!(graph.part_joins.get_part_cluster(0), 0);
        assert_eq!(graph.part_joins.get_part_cluster(1), 1);
        assert_eq!(graph.part_joins.get_part_cluster(2), 2);

        let mut graph = graph;
        graph.part_joins.perform_all_joins();

        assert_eq!(graph.part_joins.get_part_cluster(0), 0);
        assert_eq!(graph.part_joins.get_part_cluster(1), 0);
        assert_eq!(graph.part_joins.get_part_cluster(2), 0);
    }

    #[test]
    fn test_part_joins_deferred_sew() {
        let acl = indoc::indoc! {"
        == Part1 ==
        : MR(6)
        : 6 sc, mark(p1)
        FO

        == Part2 ==
        : MR(6)
        : 6 sc, mark(p2), mark(p3)
        FO

        == Part3 ==
        : MR(6)
        : 6 sc, mark(p4)
        FO

        sew(p1, p2)
        sew(p4, p3)
    "};
        let pattern = PatternBuilder::parse(acl).unwrap();
        let hook_params = HookParams {
            tip_from_fo: true,
            enforce_counts: false,
        };
        let graph = graph_construction::parse(pattern.as_iter(), hook_params).unwrap();
        assert_eq!(graph.part_joins.part_to_cluster.len(), 3);

        assert_eq!(graph.part_joins.joins.len(), 2);
        assert_eq!(graph.part_joins.joins[0].with_node, 42);
        assert_eq!(graph.part_joins.joins[0].from, 1);
        assert_eq!(graph.part_joins.joins[0].to, 0);

        assert_eq!(graph.part_joins.joins[1].with_node, 42);
        assert_eq!(graph.part_joins.joins[1].from, 2);
        assert_eq!(graph.part_joins.joins[1].to, 1);

        assert_eq!(graph.part_joins.get_part_cluster(0), 0);
        assert_eq!(graph.part_joins.get_part_cluster(1), 1);
        assert_eq!(graph.part_joins.get_part_cluster(2), 2);

        let mut graph = graph;
        graph.part_joins.perform_all_joins();

        assert_eq!(graph.part_joins.get_part_cluster(0), 0);
        assert_eq!(graph.part_joins.get_part_cluster(1), 0);
        assert_eq!(graph.part_joins.get_part_cluster(2), 0);
    }

    #[test]
    fn test_unholy_joins() {
        let acl = indoc::indoc! {"
        == Part0 ==
        : MR(6)
        mark(p0)
        == Part1 ==
        : MR(6)
        mark(p1)
        == Part2 ==
        : MR(6)
        mark(p2)
        == Part3 ==
        : MR(6)
        mark(p3)
        == Part4 ==
        : MR(6)
        mark(p4)
        == Part5 ==
        : MR(6)
        mark(p5)
        == Part6 ==
        : MR(6)
        mark(p6)
        sew(p1, p0), sew(p2, p1)
        sew(p3, p4), sew(p5, p6)
        sew(p4, p0), sew(p5, p0)
    "};
        let pattern = PatternBuilder::parse(acl).unwrap();
        let hook_params = HookParams {
            tip_from_fo: true,
            enforce_counts: false,
        };
        let graph = graph_construction::parse(pattern.as_iter(), hook_params).unwrap();
        let mut joins = graph.part_joins;

        assert_eq!(joins.part_to_cluster, vec![0, 1, 2, 3, 4, 5, 6]);
        joins.perform_next_join(); // sew(p1, p0)
        assert_eq!(joins.part_to_cluster, vec![0, 0, 2, 3, 4, 5, 6]);
        joins.perform_next_join(); // sew(p2, p1)
        assert_eq!(joins.part_to_cluster, vec![0, 0, 0, 3, 4, 5, 6]);
        joins.perform_next_join(); // sew(p3, p4)
        assert_eq!(joins.part_to_cluster, vec![0, 0, 0, 3, 3, 5, 6]);
        joins.perform_next_join(); // sew(p5, p6)
        assert_eq!(joins.part_to_cluster, vec![0, 0, 0, 3, 3, 5, 5]);
        joins.perform_next_join(); // sew(p4, p0)
        assert_eq!(joins.part_to_cluster, vec![0, 0, 0, 0, 0, 5, 5]);
        joins.perform_next_join(); // sew(p5, p0)
        assert_eq!(joins.part_to_cluster, vec![0, 0, 0, 0, 0, 0, 0]);
    }
}
