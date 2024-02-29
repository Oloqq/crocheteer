use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "debugging command")]
    Dev { num: usize },

    #[structopt(alias = "ws", about = "Run a WebSocket server for visualization")]
    WebSocket {},

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
        #[structopt(short, long)]
        is_string: bool,

        pattern: PathBuf,

        stl: Option<PathBuf>,

        #[structopt(short, long)]
        ws: bool,
    },

    #[structopt(
        aliases = &["ffp", "proto"],
        about = "Transformations starting from a proto pattern file"
    )]
    FromProtoPattern { protopat: Option<PathBuf> },
}

#[derive(StructOpt, Debug)]
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
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Command,
}
