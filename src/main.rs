// Provides the functionality for your CLI
use structopt::StructOpt;

mod app;
mod errors;

use errors::{AppResult, Error};

fn main() -> AppResult<()> {
    let app = app::App::from_args();
    let input = match &app.perhaps {
        Some(perhaps_in) => check_input(perhaps_in)?,
        None => "Default".to_owned(),
    };
    let gimme = check_input(&app.gimme)?;

    match app {
        app::App { verbose: true, .. } => println!("Gimme: {}, Input: {}", gimme, input),
        _ => println!("{} {}", gimme, input),
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
