pub mod config;

use std::error::Error;
use std::fs;

use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = search(&config.query, &contents, config.ignore_case);

    if results.len() == 0 {
        println!("No results.");
    } else {
        for line in results {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn help() {
    println!(
        "\
## Minigrep ##
Search a file for a given phrase.

Instructions for use:
minigrep <phrase> <filepath> <optional: ignore-case>

Including the ignore-case argument will perform a case-insensitive search.
Alternatively, set an IGNORE_CASE=1 environment variable to perform a case-insensitive search.
"
    )
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    if ignore_case {
        let query = query.to_lowercase();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
    } else {
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nSafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(
            vec!["Safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
