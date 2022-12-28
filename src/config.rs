use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Position of the 'ignore case' argument
    const IGNORE_CASE_INDEX: usize = 3;

    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        // User has given 'ignore case' argument or set the 'IGNORE_CASE' env var
        let ignore_case = if (args.len() >= 4
            && (args[Self::IGNORE_CASE_INDEX] == "ignore-case"
                || args[Self::IGNORE_CASE_INDEX] == "ic"))
            || (env::var("IGNORE_CASE").is_ok() && env::var("IGNORE_CASE").unwrap() == "1")
        {
            true
        } else {
            false
        };

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}
