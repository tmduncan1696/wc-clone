use std::error::Error;

pub mod cli;
use crate::cli::Cli;

pub mod counter;
use crate::counter::Counts;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let counts = Counts::from(cli);

    println!("{}", counts.to_string());

    Ok(())
}

