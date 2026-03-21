pub use crate::hook::hook_result::InitialGraph;
use crate::{
    acl::pest_parser::Pattern,
    hook::{Hook, HookParams, initializer},
};

#[allow(unused)] // TODO
mod acl;

#[allow(unused)] // TODO
mod hook;

#[allow(unused)] // TODO
mod params;

pub fn v0() -> glam::Vec3 {
    glam::Vec3::ZERO
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn parse_into_graph(acl_source: &str) -> InitialGraph {
    let syntax_result = Pattern::parse(acl_source).unwrap();
    println!("syntax: {:?}", syntax_result);
    let semantic_result = Hook::parse(syntax_result, &HookParams::default()).unwrap();
    println!("semantic: {:?}", semantic_result);
    semantic_result
}

pub fn parse_into_points(acl_source: &str) -> Vec<glam::Vec3> {
    let syntax_result = Pattern::parse(acl_source).unwrap();
    println!("syntax: {:?}", syntax_result);
    let semantic_result = Hook::parse(syntax_result, &HookParams::default()).unwrap();
    println!("semantic: {:?}", semantic_result);
    let ini = initializer::Initializer::Cylinder;
    let (nodes, _edges, _, _) = ini.apply_to(semantic_result);
    nodes.points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
