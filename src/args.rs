use std::path::PathBuf;
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

    #[structopt(short = "l", long, default_value = "pillar")]
    pub plushie: String,

    #[structopt(short = "m", long, default_value = "default")]
    pub params: String,

    #[structopt(short, long)]
    pub secondary: Option<PathBuf>,
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
