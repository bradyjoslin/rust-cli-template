// Defines your CLI interface using structopt
use structopt::StructOpt;

/// TODO:  Add description of your CLI
#[derive(StructOpt, Debug)]
#[structopt(name = "TODO: Specify CLI Name")]
pub struct App {
    /// Example required input.
    #[structopt(short, long)]
    pub gimme: String,

    /// Example optional input.
    #[structopt(short, long)]
    pub perhaps: Option<String>,

    /// Verbose output.
    #[structopt(short, long)]
    pub verbose: bool,
}
