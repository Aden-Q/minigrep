use std::{env, process};

use minigrep::Config;

fn main() {
    let config: Config = Config::parse_config(Box::new(env::args())).unwrap_or_else(|err| {
        eprintln!(
            "Problem parsing arguments: {}. Usage: grep PATTERN FILENAME",
            err
        );
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}
