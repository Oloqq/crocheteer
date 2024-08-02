use std::{fs, path::PathBuf};
pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "debugging command")]
    Dev { num: usize },

    #[structopt(alias = "ws", about = "Start a WebSocket server for visualization")]
    WebSocket(WebsocketArgs),

    #[structopt(alias = "ins", about = "Inspect a population")]
    Inspect(InspectArgs),

    #[structopt(about = "Start a server for calculating fitness")]
    Rank(RankArgs),

    #[structopt(
        aliases = &["fp", "p", "pat"],
        about = "Transformations starting from a pattern file"
    )]
    FromPattern {
        pattern: PathBuf,

        #[structopt(short, long)]
        stl: Option<PathBuf>,

        #[structopt(short, long)]
        ws: bool,
    },
}

#[derive(StructOpt, Debug)]
pub struct WebsocketArgs {
    #[structopt(short, long, default_value = "8080")]
    pub port: u16,

    /// Sets both `plushie` and `params`
    #[structopt(long)]
    pub preset: Option<String>,

    #[structopt(short = "l", long, required_unless("preset"))]
    pub plushie: Option<String>,

    #[structopt(short = "m", long, required_unless("preset"))]
    pub params: Option<String>,

    #[structopt(short, long)]
    pub secondary: Option<PathBuf>,
}

pub struct AppliedPreset<'a> {
    pub plushie: &'a str,
    pub params: &'a str,
    pub secondary: Option<PathBuf>,
}

impl WebsocketArgs {
    pub fn apply_preset(&self) -> AppliedPreset {
        let mut plushie = &self.plushie;
        let mut params = &self.params;
        let mut secondary = self.secondary.clone();

        if let Some(preset) = &self.preset {
            if plushie.is_none() {
                plushie = &self.preset;
            }
            if params.is_none() {
                params = &self.preset;
            }
            if secondary.is_none() {
                let path = format!("model_preprocessing/models/{}.json", preset);
                if fs::exists(&path).expect("filesystem to work") {
                    secondary = Some(PathBuf::from(path));
                }
            }
        }

        let plushie = plushie.as_ref().expect("Use --plushie or --preset");
        let params = params.as_ref().expect("Use --params or --preset");
        AppliedPreset {
            plushie,
            params,
            secondary,
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct RankArgs {
    // #[structopt(short, long, default_value = "8080")]
    // pub port: u16,
    #[structopt(short, long, default_value = "model_preprocessing/models/pillar.json")]
    pub goal: String,

    #[structopt(short, long, default_value = "default")]
    pub params: String,
}

#[derive(StructOpt, Debug)]
pub struct InspectArgs {
    pub popfile: String,
    pub index: usize,
}

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Command,
}
