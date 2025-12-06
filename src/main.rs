use std::error::Error;
use wc_clone::{run, cli::Cli};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::new();

    run(cli)
}
