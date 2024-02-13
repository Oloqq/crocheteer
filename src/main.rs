fn main() {
    use std::fs::OpenOptions;
    use stl_io::{Normal, Vertex};
    let mesh = [stl_io::Triangle { normal: Normal::new([1.0, 0.0, 0.0]),
                                vertices: [Vertex::new([0.0, -1.0, 0.0]),
                                            Vertex::new([0.0, 1.0, 0.0]),
                                            Vertex::new([0.0, 0.0, 0.5])]}];
    let mut file = OpenOptions::new().write(true).create_new(true).open("generated/triangle.stl").unwrap();
    stl_io::write_stl(&mut file, mesh.iter()).unwrap();
}
