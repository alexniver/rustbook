use std::{env, error::Error, fs::read_to_string, process};

use ch_12_grep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("parse args error, {}", e);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
