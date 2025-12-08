use std::collections::HashMap;
use std::hash::Hash;
use std::ops::AddAssign;

use strum::IntoEnumIterator;

use crate::cli::Cli;
use crate::command::Command;
use crate::counter::Counter;

#[derive(Debug, PartialEq)]
pub struct Counts {
    pub counts: Vec<Counter>
}

impl From<Cli> for Counts {
    fn from(cli: Cli) -> Self {
        let files: Vec<String> = cli.files;

        let mut commands: Vec<Command> = Vec::new();

        if cli.lines {
            commands.push(Command::CountLines);
        };

        if cli.words {
            commands.push(Command::CountWords);
        };

        if cli.chars {
            commands.push(Command::CountChars);
        };

        if cli.bytes {
            commands.push(Command::CountBytes);
        };

        if files.len() == 1 {
            return Counts {
                counts: vec![
                    Counter::build(files[0].clone(), commands)
                ]
            }
        };

        let mut counters: Vec<Counter> = files
            .into_iter()
            .map(|f| Counter::build(f, commands.clone()))
            .collect();

        let count_hashmaps: Vec<HashMap<Command, usize>> = counters
            .iter()
            .map(|c| c.counts.clone())
            .collect();

        let total_counts = merge_sum_vec(count_hashmaps);

        let total = Counter {
            file: "total".to_string(),
            counts: total_counts
        };

        counters.push(total);

        Counts {
            counts: counters
        }
    }
}

impl Counts {
    pub fn to_string(self) -> String {

        let mut out = String::new();
        
        let max_count = if self.counts.len() == 1 {
            self.counts[0].counts
                .clone()
                .into_iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(_k, v)| v)
                .expect("No counts calculated")
        } else {
            if let Some(x) = self.counts.last() {
                x.counts
                    .clone()
                    .into_iter()
                    .max_by(|a, b| a.1.cmp(&b.1))
                    .map(|(_k, v)| v)
                    .expect("No counts calculated")
            } else {
                0
            }
        };

        if max_count == 0 {
            eprintln!("No counts calculated");
        };

        let max_count_digits: usize = (max_count.checked_ilog10().unwrap_or(0) + 2) as usize;

        for counter in self.counts {
            for command in Command::iter() {
                if let Some(val) = &counter.counts.get(&command) {
                    out += &format!("{:>max_count_digits$}", &val.to_string()).to_string();
                }
            }
            out += &(" ".to_string() + &counter.file + "\n");
        };

        out.trim_end().to_string()

    }
}

fn merge_sum<K, V>(map1: &HashMap<K, V>, map2: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: AddAssign + Copy
{
    let mut out = map1.clone();

    for (key, value) in map2 {
        out
            .entry(key.clone())
            .and_modify(|v| *v += *value)
            .or_insert(*value);
    };

    out
}

fn merge_sum_vec<K, V>(maps: Vec<HashMap<K, V>>) -> HashMap<K, V>
where 
    K: Eq + Hash + Clone,
    V: AddAssign + Copy
{
    maps
        .iter()
        .fold(HashMap::new(), |a, b| merge_sum(&a, &b))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counter_to_string() {
        let commands: Vec<Command> = vec![Command::CountLines, Command::CountWords, Command::CountChars];
        let counts = Counter::build("test_files/poem.txt".to_string(), commands);
        
        let out = counts.to_string();

        assert_eq!(
            out,
            "   9  44 221 test_files/poem.txt"
        )
    }

    #[test]
    fn test_merge_sum() {
        let map1 = HashMap::from([
            (Command::CountLines, 9),
            (Command::CountWords, 44),
            (Command::CountChars, 221)
        ]);

        let map2 = HashMap::from([
            (Command::CountLines, 2),
            (Command::CountWords, 6),
            (Command::CountChars, 24)
        ]);

        let out = merge_sum(&map1, &map2);

        assert_eq!(
            out,
            HashMap::from([
                (Command::CountLines, 11),
                (Command::CountWords, 50),
                (Command::CountChars, 245)
            ])
        )
        
    }

    #[test]
    fn test_merge_sum_vec() {
        let map1 = HashMap::from([
            (Command::CountLines, 9),
            (Command::CountWords, 44),
            (Command::CountChars, 221)
        ]);

        let map2 = HashMap::from([
            (Command::CountLines, 2),
            (Command::CountWords, 6),
            (Command::CountChars, 24)
        ]);

        let maps = vec![map1, map2];

        let out = merge_sum_vec(maps);

        assert_eq!(
            out,
            HashMap::from([
                (Command::CountLines, 11),
                (Command::CountWords, 50),
                (Command::CountChars, 245)
            ])
        )

    }

    #[test]
    fn test_from_one_file() {
        let cli = Cli {
            files: vec!["test_files/poem.txt".to_string()],
            bytes: false,
            chars: true,
            words: true,
            lines: true
        };

        let out = Counts::from(cli);

        assert_eq!(
            out,
            Counts {
                counts: vec![
                    Counter::build("test_files/poem.txt".to_string(), vec![Command::CountLines, Command::CountWords, Command::CountChars])
                ]
            }
        )

    }

    #[test]
    fn test_from_two_files() {
        let cli = Cli {
            files: vec!["test_files/poem.txt".to_string(), "test_files/foo.txt".to_string()],
            bytes: false,
            chars: true,
            words: true,
            lines: true
        };

        let out = Counts::from(cli);

        assert_eq!(
            out,
            Counts {
                counts: vec![
                    Counter::build("test_files/poem.txt".to_string(), vec![Command::CountLines, Command::CountWords, Command::CountChars]),
                    Counter::build("test_files/foo.txt".to_string(), vec![Command::CountLines, Command::CountWords, Command::CountChars]),
                    Counter {
                        file: "total".to_string(),
                        counts: HashMap::from([
                            (Command::CountLines, 12),
                            (Command::CountWords, 49),
                            (Command::CountChars, 241)
                        ])
                    }
                ]
            }
        )

    }

    #[test]
    fn test_counts_one_file_to_string() {
        let cli = Cli {
            files: vec!["test_files/poem.txt".to_string()],
            bytes: false,
            chars: true,
            words: true,
            lines: true
        };

        let counts = Counts::from(cli);

        let out = counts.to_string();

        assert_eq!(
            out,
            "   9  44 221 test_files/poem.txt"
        )

    }

    #[test]
    fn test_counts_two_files_to_string() {
        let cli = Cli {
            files: vec!["test_files/poem.txt".to_string(), "test_files/foo.txt".to_string()],
            bytes: false,
            chars: true,
            words: true,
            lines: true
        };

        let counts = Counts::from(cli);

        let out = counts.to_string();

        assert_eq!(
            out,
            "   9  44 221 test_files/poem.txt
   3   5  20 test_files/foo.txt
  12  49 241 total"
        )
    }
}
