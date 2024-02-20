use std::path::PathBuf;
pub use structopt::StructOpt;

// TODO organize those into verbs

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(long)]
    pub dev: Option<usize>,

    #[structopt(short, long, parse(from_os_str))]
    pub pattern: Option<PathBuf>,

    #[structopt(short, long, parse(from_os_str))]
    pub stl: Option<PathBuf>,

    #[structopt(short, long)]
    /// Run a websocket server for visualization
    pub ws: bool,

    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt(long, parse(from_os_str))]
    pub protopat: Option<PathBuf>,

    #[structopt(long)]
    pub genetic: bool,

    #[structopt(long)]
    pub stdout: bool, // TEMP to compile tinygp

    #[structopt(long)]
    pub fresh: bool, // TEMP to compile tinygp

    #[structopt(long)]
    pub seed: Option<u64>, // TEMP to compile tinygp

    #[structopt(long, default_value = "0")]
    pub generations: usize, // TEMP to compile tinygp

    #[structopt(long)]
    pub suite: Option<String>, // TEMP to compile tinygp
}
