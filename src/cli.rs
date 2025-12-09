use clap::{Parser, ArgAction};

#[derive(Parser, Debug, PartialEq)]
#[command(name = "WC Clone")]
pub struct Cli {
    #[arg(value_parser, num_args = 0.., value_delimiter = ' ')]
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
        Self::parse().fix_no_commands()
    }

    fn fix_no_commands(self) -> Self {
        let has_any_commands: &bool = &(self.lines || self.words || self.chars || self.bytes);

        let out = if !has_any_commands {
            Cli {
                lines: true,
                words: true,
                chars: true,
                ..self
            }
        } else {
            self
        };

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fix_no_commands() {
        let cli = Cli {
            files: vec!["test_files/poem.txt".to_string()],
            lines: false,
            words: false,
            chars: false,
            bytes: false
        };

        let out = cli.fix_no_commands();

        assert_eq!(
            out,
            Cli {
                files: vec!["test_files/poem.txt".to_string()],
                lines: true,
                words: true,
                chars: true,
                bytes: false
            }
        );

    }
}
