use clap::{
    Parser,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
};

/// Custom Clap styling to mimic a beautiful colored help menu.
fn get_styles() -> Styles {
    let cyan = AnsiColor::Cyan.on_default();
    let green = AnsiColor::Green.on_default();
    let yellow = AnsiColor::Yellow.on_default();

    Styles::styled()
        .header(yellow | Effects::BOLD)
        .usage(yellow | Effects::BOLD)
        .literal(green)
        .placeholder(cyan)
}

#[derive(Parser, Debug)]
#[command(
    name = "matar",
    author,
    version,
    about = "A robust process termination utility.",
    long_about = None,
    styles=get_styles(),
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
