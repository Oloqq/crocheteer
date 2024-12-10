mod growing;
mod initial_cross_sections;
mod local_surface_normals;
mod utils;

#[rustfmt::skip]
mod in_execution_order {
    use super::*;

    pub use local_surface_normals::local_surface_normals_per_point;

    #[allow(unused)]
    pub use initial_cross_sections::max_dists;
    pub use initial_cross_sections::do_clustering;
    pub use initial_cross_sections::select_seeds;
    pub use initial_cross_sections::detect_initial_cross_sections;
    #[allow(unused)]
    pub use initial_cross_sections::CrossSection;

    pub use growing::grow;
    pub use growing::Part;
}

pub use in_execution_order::*;
