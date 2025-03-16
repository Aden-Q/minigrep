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
    pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let pattern: String = args[1].clone();
        let filepath: String = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config::new(pattern, filepath, ignore_case))
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filepath)?;

    let result: Vec<&str> = match config.ignore_case {
        true => search(&config.pattern, &contents),
        false => search_case_insensitive(&config.pattern, &contents),
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.contains(pattern) {
            res.push(line);
        }
    }

    res
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    let pattern: &str = &pattern.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(pattern) {
            res.push(line);
        }
    }

    res
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
