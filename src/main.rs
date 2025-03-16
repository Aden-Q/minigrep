use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::parse_config(&args).unwrap_or_else(|err| {
        eprintln!(
            "Problem parsing arguments: {}. Usage: grep PATTERN FILENAME",
            err
        );
        process::exit(1);
    });

    // debug only
    println!("Searching for {}", config.pattern);
    println!("In file {}", config.filepath);

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}
