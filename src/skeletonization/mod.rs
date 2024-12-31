mod growing;
mod initial_cross_sections;
mod local_surface_normals;
mod part_selection;
mod utils;

#[rustfmt::skip]
mod in_execution_order {
    use super::*;

    pub use utils::Connectivity;

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

use std::time::Instant;

pub use in_execution_order::*;

pub fn get_skelet(
    plushie: &crate::plushie::Plushie,
    cluster_num: usize,
    must_include_points: f32,
    allowed_overlap: f32,
    perf: &mut Option<crate::plushie::perf::Iteration>,
) -> Vec<crate::common::Point> {
    log::trace!("getting skelet...");
    let start = Instant::now();

    let cloud = &plushie.nodes.points;
    let connectivity = Connectivity::new(&plushie.edges);

    log::trace!("getting normals...");
    let surface_normals = local_surface_normals_per_point(cloud);
    let t_normals = start.elapsed();

    log::trace!("initial cross section...");
    let cross_sections =
        detect_initial_cross_sections(cloud, &connectivity, cluster_num, &surface_normals);
    let t_sections = start.elapsed();

    log::trace!("growing...");
    let parts: Vec<Part> = grow(cloud, &connectivity, cross_sections, &surface_normals);
    let t_growing = start.elapsed();

    log::trace!("all parts: {}...", parts.len());
    let parts = select_parts(
        parts,
        PartSelectionParams::new(cloud.len(), must_include_points, allowed_overlap),
        cloud,
    );
    let t_selection = start.elapsed();
    log::trace!("selected parts: {}", parts.len());

    println!(
        "normals: {:?}, initial_cross: {:?}, growing: {:?}, part_selection: {:?}, skeletonization: {:?}",
        t_normals,
        t_sections - t_normals,
        t_growing - t_sections,
        t_selection - t_growing,
        t_selection
    );

    if let Some(perf) = perf {
        perf.normals = t_normals;
        perf.initial_cross = t_sections - t_normals;
        perf.growing = t_growing - t_sections;
        perf.part_selection = t_selection - t_growing;
        perf.skeletonization = t_selection;
    }

    parts
        .iter()
        .flat_map(|p| p.sections.iter().map(|s| s.center.into()))
        .collect()
}
