use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "debugging command")]
    Dev { num: usize },

    #[structopt(alias = "ws", about = "Run a WebSocket server for visualization")]
    WebSocket(WebsocketArgs),

    #[structopt(alias = "ins", about = "Inspect a population")]
    Inspect(InspectArgs),

    #[structopt(
        alias = "gen",
        about = "Run a specified benchmark for genetic algorithms"
    )]
    Genetic(GeneticArgs),

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
#[allow(unused)]
pub struct GeneticArgs {
    #[structopt(long)]
    pub stdout: bool,

    #[structopt(short, long)]
    pub fresh: bool,

    #[structopt(short, long)]
    pub seed: Option<u64>,

    #[structopt(short, long, default_value = "0")]
    pub generations: usize,

    #[structopt(long)]
    pub save_stl: bool,

    /// Include full information in generation reports. Expensive.
    #[structopt(short, long)]
    pub debug: bool,

    pub suite: String,
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
