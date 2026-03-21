use crate::acl::pest_parser::Pattern;

mod acl;

pub fn v0() -> glam::Vec3 {
    glam::Vec3::ZERO
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn parse(acl_source: &str) {
    let result = Pattern::parse(acl_source);
    println!("parsed: {:?}", result);
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
