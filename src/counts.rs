use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::cli::Cli;

#[derive(Debug, PartialEq, Hash, Eq, Clone, EnumIter)]
pub enum Command {
    CountLines,
    CountWords,
    CountChars,
    CountBytes
}

#[derive(Debug, PartialEq)]
pub struct Counts {
    file: String,
    counts: HashMap<Command, usize>
}

impl From<Cli> for Counts {
    fn from(cli: Cli) -> Self {
        let file: String = cli.file;

        let mut commands: Vec<Command> = Vec::new();

        let lines: bool = if cli.lines || cli.words || cli.chars || cli.bytes { cli.lines } else { true };
        let words: bool = if cli.lines || cli.words || cli.chars || cli.bytes { cli.words } else { true };
        let chars: bool = if cli.lines || cli.words || cli.chars || cli.bytes { cli.chars } else { true };
        let bytes: bool = if cli.lines || cli.words || cli.chars || cli.bytes { cli.bytes } else { false };

        if lines {
            commands.push(Command::CountLines);
        };

        if words {
            commands.push(Command::CountWords);
        };

        if chars {
            commands.push(Command::CountChars);
        };

        if bytes {
            commands.push(Command::CountBytes);
        };

        Counts::build(file, commands)
    }
}

impl Counts {
    fn build(file: String, commands: Vec<Command>) -> Self {
        let contents: String = std::fs::read_to_string(&file)
            .unwrap_or_else(|_err| {
                eprintln!("Could not read file: {}", file);
                std::process::exit(1);
            });

        let counts = count(&contents, commands);

        Counts {
            file,
            counts
        }
    }

    pub fn to_string(self) -> String {
        let mut out = String::from("");

        let max_count = self.counts
            .clone()
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(_k, v)| v)
            .expect("No counts calcualted");

        let max_count_digits: usize = (max_count.checked_ilog10().unwrap_or(0) + 2) as usize;

        for command in Command::iter() {
            if let Some(val) = &self.counts.get(&command) {
                out += &format!("{:>max_count_digits$}", &val.to_string()).to_string();
            }
        };

        out += &(" ".to_string() + &self.file);

        out
    }
}

fn count_lines(s: &str) -> usize {
    s.lines().count()
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn count_chars(s: &str) -> usize {
    s.chars().count()
}

fn count_bytes(s: &str) -> usize {
    s.len()
}

fn count_from_command(s: &str, command: Command) -> usize {
    match command {
        Command::CountLines => count_lines(&s),
        Command::CountWords => count_words(&s),
        Command::CountChars => count_chars(&s),
        Command::CountBytes => count_bytes(&s)
    }
}

fn count(s: &str, commands: Vec<Command>) -> HashMap<Command, usize> {
    commands
        .into_iter()
        .map(|command| (command.clone(), count_from_command(&s, command.clone())))
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_lines() {
        let content: String = std::fs::read_to_string("test_files/poem.txt").unwrap();

        let out = count_lines(&content);

        assert_eq!(
            out,
            9
        );
    }

    #[test]
    fn test_count_words() {
        let content: String = std::fs::read_to_string("test_files/poem.txt").unwrap();

        let out = count_words(&content);

        assert_eq!(
            out,
            44
        );
    }

    #[test]
    fn test_count_chars() {
        let content: String = std::fs::read_to_string("test_files/poem.txt").unwrap();

        let out = count_chars(&content);

        assert_eq!(
            out,
            221
        );
    }

    #[test]
    fn test_build() {
        let commands: Vec<Command> = vec![Command::CountLines, Command::CountWords, Command::CountChars];
        let out = Counts::build("test_files/poem.txt".to_string(), commands);

        let file = "test_files/poem.txt".to_string();
        let counts = HashMap::from([
            (Command::CountLines, 9),
            (Command::CountWords, 44),
            (Command::CountChars, 221)
        ]);

        assert_eq!(
            out,
            Counts {
                file,
                counts
            }
        )
    }

    #[test]
    fn test_from() {
        let cli: Cli = Cli {
            file: "test_files/poem.txt".to_string(),
            bytes: false,
            chars: false,
            words: false,
            lines: false
        };

        let out: Counts = Counts::from(cli);

        let file = "test_files/poem.txt".to_string();
        let counts = HashMap::from([
            (Command::CountLines, 9),
            (Command::CountWords, 44),
            (Command::CountChars, 221)
        ]);

        assert_eq!(
            out,
            Counts {
                file,
                counts
            }
        )
    }

    #[test]
    fn test_to_string() {
        let commands: Vec<Command> = vec![Command::CountLines, Command::CountWords, Command::CountChars];
        let counts = Counts::build("test_files/poem.txt".to_string(), commands);
        
        let out = counts.to_string();

        assert_eq!(
            out,
            "   9  44 221 test_files/poem.txt"
        )
    }
}
