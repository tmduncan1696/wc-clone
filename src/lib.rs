use std::error::Error;

pub mod cli;
use crate::cli::Cli;

pub mod counter;
use crate::counter::Counter;


pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let counter = Counter::from(cli);

    println!("{}", counter.to_string());

    Ok(())
}

