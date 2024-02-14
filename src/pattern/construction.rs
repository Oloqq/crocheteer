use std::fs;
use std::path::PathBuf;

use super::{Pattern, Stitch};

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

pub struct PatternBuilder {
    starting_ring: usize,
    rounds: Vec<Vec<Stitch>>,
    error: Option<(usize, String)>,
}

impl PatternBuilder {
    pub fn new(starting_ring: usize) -> Self {
        Self {
            starting_ring,
            rounds: vec![],
            error: None,
        }
    }

    pub fn full_rounds(mut self, num: usize) -> Self {
        let stitches = match self.rounds.last() {
            Some(round) => round.len(),
            None => self.starting_ring,
        };

        for _ in 0..num {
            self.rounds
                .push((0..stitches).map(|_| Stitch::Single).collect());
        }
        self
    }

    pub fn build(self) -> Result<Pattern, (usize, String)> {
        if let Some(error) = self.error {
            return Err(error);
        }
        let last_round = match self.rounds.last() {
            Some(round) => round,
            None => return Err((0, "Pattern must have at least one round".into())),
        };

        Ok(Pattern {
            starting_circle: self.starting_ring,
            ending_circle: last_round.len(),
            rounds: self.rounds,
        })
    }
}
