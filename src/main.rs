// Provides the functionality for your CLI
use structopt::StructOpt;

mod app;
mod errors;

use errors::{AppResult, Error};

fn main() -> AppResult<()> {
    let app = app::App::from_args();
    let input = check_input(&app.perhaps)?;
    let gimme = check_input(&app.gimme)?;
    let positional = check_input(&app.positional)?;

    match app {
        app::App { verbose: true, .. } => println!(
            "Gimme: {}, Input: {}, Positional: {}",
            positional, gimme, input
        ),
        _ => println!("{}\n{}\n{}", positional, gimme, input),
    }

    Ok(())
}

fn check_input(input: &str) -> AppResult<String> {
    if input == "BadInput" {
        Err(Error::BadInput)
    } else {
        Ok(input.to_owned())
    }
}
