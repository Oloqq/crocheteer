use crate::meshes::*;

mod meshes;

use std::{thread, time};

fn main() {
    use std::fs::OpenOptions;

    // let mesh = two_triangles();
    // let mesh = square();
    for i in 3..10 {
        let mesh = ring(i);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("generated/test.stl")
            .unwrap();
        stl_io::write_stl(&mut file, mesh.iter()).unwrap();

        let period = time::Duration::from_millis(500);
        thread::sleep(period);
    }
}
