use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub filepath: String,
}

impl Config {
    pub fn new(pattern: String, filepath: String) -> Self {
        Config { pattern, filepath }
    }

    // parse config from arguments
    pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let pattern: String = args[1].clone();
        let filepath: String = args[2].clone();

        Ok(Config::new(pattern, filepath))
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;
    println!("With text:\n{contents}");

    Ok(())
}
