use crocheteer::Project;
use std::path::PathBuf;

fn main() {
    let (project, file): (Project, Option<PathBuf>) = match StartMode::from_args() {
        StartMode::DefaultProject => (Project::default(), None),
        StartMode::Example(name) => (
            Project::from_example(&name).unwrap_or_else(|| {
                eprintln!("Unknown example '{name}'");
                std::process::exit(1);
            }),
            None,
        ),
        StartMode::Open(path) => (
            Project::from_file(&path).unwrap_or_else(|e| {
                eprintln!("Failed to open '{}': {e}", path.display());
                std::process::exit(1);
            }),
            Some(path),
        ),
    };

    crocheteer::app(project, file).run();
}

pub enum StartMode {
    DefaultProject,
    Example(String),
    Open(PathBuf),
}

impl StartMode {
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().skip(1).collect();
        match args.as_slice() {
            [] => Self::DefaultProject,
            [flag, value] if flag == "--example" || flag == "-e" => Self::Example(value.clone()),
            [flag, value] if flag == "--open" || flag == "-o" => Self::Open(PathBuf::from(value)),
            _ => {
                eprintln!(
                    "Usage:\n  {} [--example <name> | --open <path>]",
                    std::env::args().next().unwrap_or_default()
                );
                std::process::exit(1);
            }
        }
    }
}
