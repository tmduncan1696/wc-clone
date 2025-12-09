use std::collections::HashMap;
use std::error::Error;

use strum::IntoEnumIterator;

use crate::command::Command;


#[derive(Debug, PartialEq)]
pub struct Counter {
    pub file: String,
    pub counts: HashMap<Command, usize>
}

impl Counter {
    pub fn build(file: String, commands: Vec<Command>) -> Self {
        let contents: String = read_file(&file)
            .unwrap_or_else(|_err| {
                eprintln!("Could not read file: {}", file);
                std::process::exit(1);
            });

        let counts = count(&contents, commands);

        Counter {
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
            .expect("No counts calculated");

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

fn read_file(file: &String) -> Result<String, Box<dyn Error>> {
    if *file == String::from("-") || *file == String::from("") {
        Ok(std::io::stdin().lines().collect::<Result<String, _>>()? + "\n")
    } else {
        Ok(std::fs::read_to_string(file)?)
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
        let out = Counter::build("test_files/poem.txt".to_string(), commands);

        let file = "test_files/poem.txt".to_string();
        let counts = HashMap::from([
            (Command::CountLines, 9),
            (Command::CountWords, 44),
            (Command::CountChars, 221)
        ]);

        assert_eq!(
            out,
            Counter {
                file,
                counts
            }
        )
    }
}
