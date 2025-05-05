use std::error::Error;
use std::fs;
use std::env;
use regex::Regex;
use colored::*;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let _ = Regex::new(&query).map_err(|_| "Invalid regex pattern")?;

        Ok(Config { 
            query, 
            file_path, 
            ignore_case 
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)?
    } else {
        search(&config.query, &contents)?
    };

    for line in results {
        let highlighted = highlight_match(&line, &config.query, config.ignore_case);
        println!("{highlighted}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, regex::Error> {
    let re = Regex::new(query)?;
    Ok(contents
        .lines()
        .filter(|line| re.is_match(line))
        .collect())
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Result<Vec<&'a str>, regex::Error> {
    let re = Regex::new(&format!("(?i){}", query))?;
    Ok(contents
        .lines()
        .filter(|line| re.is_match(line))
        .collect())
}

fn highlight_match(line: &str, query: &str, ignore_case: bool) -> String {
    let pattern = if ignore_case {
        format!("(?i){}", regex::escape(query))
    } else {
        regex::escape(query)
    };

    let re = Regex::new(&pattern).unwrap();

    re.replace_all(line, |caps: &regex::Captures| {
        caps[0].red().bold().to_string()
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(&query, contents).unwrap()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(&query, contents).unwrap()
        );
    }

    #[test]
fn regex_search_dot_star() {
    let query = "R.*t"; // Should match "Rust"
    let contents = "\
Rust:
Trust me.";

    assert_eq!(
        vec!["Rust:"], 
        search(query, contents).unwrap()
    );
}

#[test]
fn regex_case_insensitive_alt() {
    let query = "r.*t"; // Should match both Rust and Trust
    let contents = "\
Rust:
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents).unwrap()
    );
}
}