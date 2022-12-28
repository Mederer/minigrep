use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Position of the 'ignore case' argument
    const IGNORE_CASE_INDEX: usize = 3;
    const IGNORE_CASE_EXPECTED_VALUE: &str = "1";

    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let ignore_case = Self::ignore_case(&args);

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }

    // True if user has given 'ignore-case' or 'ic' argument or set the 'IGNORE_CASE' env var
    fn ignore_case(args: &[String]) -> bool {
        if (args.len() >= 4
            && (args[Self::IGNORE_CASE_INDEX] == "ignore-case"
                || args[Self::IGNORE_CASE_INDEX] == "ic"))
            || (env::var("IGNORE_CASE").is_ok()
                && env::var("IGNORE_CASE").unwrap() == Self::IGNORE_CASE_EXPECTED_VALUE)
        {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let config = Config::build(&vec![String::from("_"), String::from("_")]);
        assert!(config.is_err());
    }

    #[test]
    fn test_build_2() {
        let config = Config::build(&vec![
            String::from("_"),
            String::from("to"),
            String::from("poem.txt"),
        ]);
        assert!(config.is_ok());
    }

    #[test]
    fn test_ignore_case() {
        assert!(Config::ignore_case(&vec![
            String::from("_"),
            String::from("_"),
            String::from("_"),
            String::from("ignore-case")
        ]));
    }

    #[test]
    fn test_ignore_case_2() {
        assert!(Config::ignore_case(&vec![
            String::from("_"),
            String::from("_"),
            String::from("_"),
            String::from("ic")
        ]));
    }

    #[test]
    fn test_ignore_case_3() {
        assert!(!Config::ignore_case(&vec![
            String::from("_"),
            String::from("_"),
            String::from("_")
        ]));
    }
}
