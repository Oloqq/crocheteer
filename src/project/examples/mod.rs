pub mod dev;
pub mod public;

impl super::Project {
    pub fn from_example(name: &str) -> Option<Self> {
        match name {
            "grzib" | "mushroom" => public::mushroom(),
            "grzib_no_slf" | "mushroom_no_slf" => dev::mushroom_no_slf(),
            "one_by_one" | "obo" => dev::one_by_one(),
            "code_highlights" => dev::code_highlights(),
            "frog" => dev::frog(),
            "sewn_parts" => dev::sewn_parts(),
            "sewn_parts_one_by_one" => dev::sewn_parts_one_by_one(),
            "unconnected_parts" => dev::unconnected_parts(),
            _ => return None,
        }
        .into()
    }
}
