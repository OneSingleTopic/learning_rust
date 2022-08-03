use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing the arguent : {}", error);
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Problem reading the file : {}", error);
        process::exit(1);
    };
}
