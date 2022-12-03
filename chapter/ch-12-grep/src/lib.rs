use std::{env, error::Error, fs::read_to_string};

pub struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enouth params");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: &args[1],
            file_path: &args[2],
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(config.file_path)?;
    let mut result = vec![];
    if config.ignore_case {
        for line in search_case_insensitive(config.query, &contents) {
            result.push(line);
        }
    } else {
        for line in search_case_sensitive(config.query, &contents) {
            result.push(line);
        }
    }

    println!("{:?}", result);

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const contents: &str = "\
Rust:
safe, fast, productive.
Peek Three.
Trust me.";

    #[test]
    fn case_sensitive() {
        let query = "duct";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
