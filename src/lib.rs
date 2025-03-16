use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(pattern: String, filepath: String, ignore_case: bool) -> Self {
        Config {
            pattern,
            filepath,
            ignore_case,
        }
    }

    // parse config from arguments
    pub fn parse_config(
        mut args: Box<dyn Iterator<Item = String>>,
    ) -> Result<Config, &'static str> {
        args.next();

        let pattern: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing pattern"),
        };

        let filepath: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing filepath"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config::new(pattern, filepath, ignore_case))
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filepath)?;

    let result: Vec<&str> = match config.ignore_case {
        false => search(&config.pattern, &contents),
        true => search_case_insensitive(&config.pattern, &contents),
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line: &&str| line.contains(pattern))
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .map(|line: &str| line.to_lowercase())
        .enumerate()
        .filter(|(_, line)| line.contains(&pattern.to_lowercase()))
        .map(|(i, _)| contents.lines().nth(i).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(pattern, contents)
        );
    }
}
