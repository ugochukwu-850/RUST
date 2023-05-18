use std::env;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut recs = Vec::new();

    for line in contents.lines() {
        // iterate over lines susing the .lines methods
        if line.contains(query) {
            //if line has any of the query add that query to the vec
            recs.push(line);
        }
    }

    recs
}

pub fn search_with_iterator_adaptors<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
    }
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /*pub fn build(args: &[String]) -> Result<Config, &'static str> {
    Updating the impl to take in aniter
    */
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        /*
            We’ve updated the signature of the Config::build function so the parameter args has a
        generic type with the trait bounds impl Iterator<Item = String> instead of &[String] . This
        usage of the impl Trait syntax we discussed in the “Traits as Parameters” section of Chapter
        10 means that args can be any type that implements the Iterator type and returns String
        items.Because we’re taking ownership of args and we’ll be mutating args by iterating over it, we
        can add the mut keyword into the specification of the args parameter to make it mutable.*/
        
        //each time you call next you re-updating args 
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /*
    since we have the resust OK value of () we need to return the OK(()) <== like that
    using  Box <dyn Error> to be able to return an error or result to main
    */
    let contents = fs::read_to_string(config.file_path)?;
    //the question mark would return an error to the caller function rather than panic
    // if there is one and return a Ok value is there is
    //..print
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}
