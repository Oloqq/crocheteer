mod growing;
mod initial_cross_sections;
mod local_surface_normals;
mod part_selection;
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

    pub use part_selection::sort_by_cost;
    pub use part_selection::select_parts;
    pub use part_selection::PartSelectionParams;
}

pub use in_execution_order::*;

pub fn get_skelet(
    plushie: &crate::plushie::Plushie,
    cluster_num: usize,
    must_include_points: f32,
    allowed_overlap: f32,
) -> Vec<crate::common::Point> {
    println!("getting skelet...");
    let cloud = &plushie.nodes.points;
    let edges = &plushie.edges;
    println!("getting normals...");
    let surface_normals = local_surface_normals_per_point(cloud);
    println!("initial cross section...");
    let cross_sections = detect_initial_cross_sections(cloud, edges, cluster_num, &surface_normals);
    println!("growing...");
    let parts: Vec<Part> = grow(cloud, edges, cross_sections, &surface_normals);
    println!("all parts: {}...", parts.len());
    let parts = select_parts(
        parts,
        PartSelectionParams::new(cloud.len(), must_include_points, allowed_overlap),
    );
    println!("selected parts: {}", parts.len());

    parts
        .iter()
        .flat_map(|p| p.sections.iter().map(|s| s.center.into()))
        .collect()
}
