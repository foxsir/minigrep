use std::{env, process};

use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
}
