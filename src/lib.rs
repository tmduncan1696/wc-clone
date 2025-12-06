use std::error::Error;

pub mod cli;
use crate::cli::Cli;

pub mod counts;
use crate::counts::Counts;


pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let counts = Counts::from(cli);

    println!("{}", counts.to_string());

    Ok(())
}

