mod initial_cross_sections;
mod local_surface_normals;

#[rustfmt::skip]
mod in_execution_order {
    use super::*;

    pub use local_surface_normals::local_surface_normals_per_point;

    // pub use initial_cross_sections::do_clustering;
    // pub use initial_cross_sections::detect_initial_cross_sections;
}

pub use in_execution_order::*;
