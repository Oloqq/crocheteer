#![allow(unused)]

pub mod rstarcomp;

/// Comparison module handles comparing shapes, thus providing a fitness function
use crate::common::*;
// mod kdtree; note to self: crate `kiddo` depends on nightly features
use std::error::Error;
use std::fs::OpenOptions;
use stl_io::IndexedMesh;

pub trait Comparator {
    fn with_basis(nodes: &Vec<Point>) -> Self;
    fn judge(&self, nodes: &Vec<Point>) -> f32;
}

fn load_stl(filepath: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).open(filepath)?;
    let stl = stl_io::read_stl(&mut file).unwrap();
    let points = stl
        .vertices
        .iter()
        .map(|v| Point::new(v[0], v[2], v[1]))
        .collect();
    Ok(points)
}
