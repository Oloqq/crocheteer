use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(long)]
    pub dev: Option<usize>,

    #[structopt(short, long, parse(from_os_str))]
    pub show: Option<PathBuf>,

    #[structopt(short, long, parse(from_os_str), default_value = "crochet_output.stl")]
    pub output: PathBuf,

    #[structopt(short, long)]
    pub verbose: bool,
}
