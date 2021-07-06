use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_with_valid_args_returns_config() {
        let expected = Config {
            query: "arg2".to_string(),
            filename: "arg3".to_string(),
        };

        let args = ["arg1".to_string(), "arg2".to_string(), "arg3".to_string()];
        let received = Config::new(&args).unwrap();

        assert_eq! { received, expected }
    }

    #[test]
    fn config_new_with_too_few_args_returns_error() {
        let args = ["arg1".to_string(), "arg2".to_string()];
        let received = Config::new(&args);

        assert!(received.is_err());
    }

    #[test]
    fn run_with_valid_file_path_succeeds() -> Result<(), Box<dyn Error>> {
        let args = [
            "first".to_string(),
            "second".to_string(),
            "testdata/poem.txt".to_string(),
        ];
        let config = Config::new(&args)?;

        run(config)
    }

    #[test]
    fn run_with_invalid_file_path_returns_error() {
        let args = [
            "first".to_string(),
            "second".to_string(),
            "invalid/path".to_string(),
        ];
        let config = Config::new(&args).unwrap();

        if let Ok(_) = run(config) {
            panic!("expected run to return an error");
        }
    }

    #[test]
    fn search_case_sensitive_works() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_case_insensitive_works() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
