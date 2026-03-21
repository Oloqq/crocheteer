use crate::{
    acl::pest_parser::Pattern,
    hook::{Hook, HookParams},
};

#[allow(unused)] // TODO
mod acl;

#[allow(unused)] // TODO
mod hook;

pub fn v0() -> glam::Vec3 {
    glam::Vec3::ZERO
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn parse(acl_source: &str) {
    let syntax_result = Pattern::parse(acl_source).unwrap();
    println!("syntax: {:?}", syntax_result);
    let semantic_result = Hook::parse(syntax_result, &HookParams::default());
    println!("semantic: {:?}", semantic_result);
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
