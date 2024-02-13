use std::fs;
use std::path::PathBuf;

use super::Pattern;

impl Pattern {
    pub fn from_file(path: PathBuf) -> Self {
        println!("{path:?}");
        let content = fs::read_to_string(&path).expect("File not found");
        let extension = path.extension().expect("Unrecognized format");
        match extension.to_str().unwrap() {
            "yaml" => Self::from_yaml_str(content.as_str()),
            _ => panic!("Unrecognized format"),
        }
    }

    pub fn from_yaml_str(content: &str) -> Self {
        serde_yaml::from_str(&content).expect("Could not parse yaml into pattern")
    }
}
