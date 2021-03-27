// Defines your CLI interface using structopt
use structopt::StructOpt;

/// TODO:  Replace comment with description of your CLI
#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
pub struct App {
    /// Example required input.
    #[structopt(short, long)]
    pub gimme: String,

    /// Example optional input.
    #[structopt(short, long, default_value)]
    pub perhaps: String,

    /// Verbose output.
    #[structopt(short, long)]
    pub verbose: bool,

    /// Positional input. Required.
    pub positional: String,
}
