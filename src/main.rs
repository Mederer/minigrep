use std::env;
use std::process;

use minigrep::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        minigrep::help();
        process::exit(0);
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(2);
    };
}
