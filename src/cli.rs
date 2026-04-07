use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "matar",
    author,
    version,
    about = "A robust process termination utility.",
    long_about = None
)]
pub struct Args {
    /// The name or pattern of the process to terminate
    #[arg(required = true)]
    pub target: String,

    /// Do not perform the second 'deep clean' pass
    #[arg(short, long)]
    pub fast: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
