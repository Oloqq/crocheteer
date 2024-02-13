#[allow(unused)]
use crate::meshes_sandbox::*;

extern crate nalgebra as na;

mod common;
mod meshes_sandbox;
mod plushie;

use common::*;
use plushie::Plushie;

fn main() {
    #[rustfmt::skip]
    let points = vec![
        Point::origin(),

        Point::new(-1.0, 1.0, -1.0),
        Point::new(1.0, 1.0, -1.0),
        Point::new(1.0, 1.0, 1.0),
        Point::new(-1.0, 1.0, 1.0),

        Point::new(-1.0, 2.0, -1.0),
        Point::new(1.0, 2.0, -1.0),
        Point::new(1.0, 2.0, 1.0),
        Point::new(-1.0, 2.0, 1.0),

        Point::new(0.0, 3.0, 0.0),
    ];

    #[rustfmt::skip]
    let edges = vec![
        // 0 ->
        vec![1, 2, 3, 4],
        // 1 ->
        vec![2, 5],
        // 2 ->
        vec![3, 6],
        // 3 ->
        vec![4, 7],
        // 4 ->
        vec![5, 8],
        // 5 ->
        vec![6, 9],
        // 6 ->
        vec![7, 9],
        // 7 ->
        vec![8, 9],
        // 8 ->
        vec![9],
        // 9 ->
        vec![],
    ];

    let p = Plushie::new(1, points, edges);
    save(p.to_mesh());

    // check_hot_reload()
    // save(square())
}
