use crate::meshes::*;

mod meshes;

fn main() {
    use std::fs::OpenOptions;

    // let mesh = two_triangles();
    // let mesh = square();
    let mesh = ring(6);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("generated/test.stl")
        .unwrap();
    stl_io::write_stl(&mut file, mesh.iter()).unwrap();
}
