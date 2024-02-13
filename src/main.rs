#[allow(unused)]
use crate::meshes_sandbox::*;

extern crate nalgebra as na;

mod common;
mod meshes_sandbox;
mod pattern;
mod plushie;

use common::*;
use pattern::Pattern;
use plushie::Plushie;

fn main() {
    // let mut plushie = diamond_plushie_direct();
    let mut plushie = Plushie::from_pattern(Pattern::tmp_diamond());

    save(
        "generated/from_pattern/before_stuffing.stl",
        plushie.to_mesh(),
    );
    plushie.stuff();
    save(
        "generated/from_pattern/after_stuffing.stl",
        plushie.to_mesh(),
    );
    plushie.stuff();
    save(
        "generated/from_pattern/after_stuffing_again.stl",
        plushie.to_mesh(),
    );
}

#[allow(unused)]
fn diamond_plushie_direct() -> Plushie {
    #[rustfmt::skip]
    let points = vec![
        Point::origin(),
        Point::new(0.0, 3.0, 0.0),

        Point::new(-1.0, 1.0, -1.0),
        Point::new(1.0, 1.0, -1.0),
        Point::new(1.0, 1.0, 1.0),
        Point::new(-2.0, 1.0, 0.5),

        Point::new(-1.0, 2.0, -1.0),
        Point::new(1.0, 2.0, -1.0),
        Point::new(1.0, 2.0, 1.0),
        Point::new(-2.0, 2.0, 0.5),

    ];

    #[rustfmt::skip]
    let edges = vec![
        // 0 ->
        vec![2, 3, 4, 5],
        // 1 ->
        vec![6, 7, 8, 9],
        // 2 ->
        vec![3, 6],
        // 3 ->
        vec![4, 7],
        // 4 ->
        vec![5, 8],
        // 5 ->
        vec![6, 9],
        // 6 ->
        vec![7],
        // 7 ->
        vec![8],
        // 8 ->
        vec![9],
        // 9 ->
        vec![],

    ];

    Plushie::new(2, points, edges)
}
