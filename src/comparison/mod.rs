#![allow(unused)]

/// Comparison module handles comparing shapes, thus providing a fitness function
use crate::common::*;
// mod kdtree; note to self: crate `kiddo` depends on nightly features
pub mod rstarcomp;

trait Comparator {
    fn with_basis(nodes: &Vec<V>) -> Self;
    fn judge(&self, nodes: &Vec<V>) -> f32;
}

// fn load_stl(filepath: &str) -> Vec<V> {

// }
