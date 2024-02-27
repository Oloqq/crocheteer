use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "bruh")]
pub enum Command {
    Dev {
        num: usize,
    },
    #[structopt(alias = "ws")]
    WebSocket {},
    #[structopt(alias = "gen")]
    Genetic(GeneticArgs),
    FromPattern {
        pattern: PathBuf,

        stl: Option<PathBuf>,
        ws: Option<PathBuf>,
    },
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

    pub suite: String,
}

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(long, parse(from_os_str))]
    pub protopat: Option<PathBuf>,

    #[structopt(subcommand)]
    pub cmd: Command,
}
