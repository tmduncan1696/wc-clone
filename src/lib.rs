use std::error::Error;

pub mod cli;
pub mod command;
pub mod counter;
pub mod counts;

use crate::cli::Cli;
use crate::counts::Counts;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let counts = Counts::from(cli);

    println!("{}", counts.to_string());

    Ok(())
}

