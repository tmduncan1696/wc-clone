use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(name = "Cat Clone")]
pub struct Cli {
    #[arg(value_parser, num_args = 1.., value_delimiter = ' ')]
    pub files: Vec<String>,

    #[arg(short = 'c', long, action = ArgAction::SetTrue)]
    pub bytes: bool,

    #[arg(short = 'm', long, action = ArgAction::SetTrue)]
    pub chars: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    pub words: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    pub lines: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
