use std::env;
use regex::RegexBuilder;
use std::path::PathBuf;
use glob::glob;

/// Represents a configuration of the app.
/// It consists of the arguments given to the program
#[derive(Debug)]
pub struct Grep {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
    pub is_regexp: bool,
}

impl Grep {
    pub fn construct_grep_from_args() -> Grep {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            panic!("2 arguments are required")
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let is_regexp = env::var("IS_REGEXP").is_ok();
        let query = args[1].clone();
        let filename = args[2].clone();
        Grep { query, filename, ignore_case, is_regexp }
    }
}

/// Returns the names of files and directories by given pattern
/// # Examples
/// ```
/// globgrep::get_paths("*.txt"); // returns all txt files
/// globgrep::get_paths("src/*"); // returns everything in the src dir
/// ```
pub fn get_paths(pattern: &str) -> impl Iterator<Item=PathBuf> {
    glob(pattern).unwrap().map(|path| path.unwrap())
}

/// Calls `get_paths` and filters out only the files
pub fn get_files(pattern: &str) -> impl Iterator<Item=PathBuf> {
    return get_paths(pattern).filter(|path| path.is_file());
}

/// Runs `matcher` against each line of `contents`.
/// Returns the found lines.
pub fn search<'a, F>(contents: &'a str, matcher: F)
                      -> impl Iterator<Item=(usize, &'a str)>
    where F: Fn(&str) -> bool + 'a {
    contents
        .lines()
        .enumerate()
        .filter(move |(_i, line)| matcher(line))
}

/// Builds a configured function that compares strings
/// # Examples
/// ```use globgrep::*;
/// let (filename, query, is_regexp) = (String::new(), String::new(), false);
/// let matcher = build_matcher(&Grep { filename, query, is_regexp, ignore_case: false }, "hello");
/// assert!(matcher("Hello world"));
/// assert!(!matcher("some text"));
/// ```
pub fn build_matcher<'a>(grep: &Grep, q: &'a str) -> Box<dyn Fn(&str) -> bool + 'a> {
    if grep.is_regexp {
        let re = RegexBuilder::new(q).case_insensitive(grep.ignore_case).build().unwrap();
        return Box::new(move |s: &str| -> bool { re.is_match(s) });
    }

    if grep.ignore_case {
        let q = q.to_lowercase();
        Box::new(move |s: &str| -> bool { s.to_lowercase().contains(&q) })
    } else {
        Box::new(move |s: &str| -> bool { s.contains(q) })
    }
}

#[cfg(test)]
mod tests {
    use super::{build_matcher, Grep, search};

    #[test]
    fn search_finds_second_line() {
        assert_eq!(search("hello\nworld", |s| s == "world").next().unwrap(), (1, "world"));
    }

    #[test]
    fn matcher_default() {
        let matcher = build_matcher(&Grep { filename: String::new(), query: String::new(), ignore_case: false, is_regexp: false },
                                    "hello World");

        assert!(!matcher("hello"), "should not contain 'hello World'");
        assert!(!matcher("hello 123 World"));
        assert!(!matcher("hello world and more"), "should not contain 'hello World'");
        assert!(!matcher("unknown"));

        assert!(matcher("hello World and more"), "should contain 'hello World'");
    }

    #[test]
    fn matcher_ignore_case() {
        let matcher = build_matcher(&Grep { filename: String::new(), query: String::new(), ignore_case: true, is_regexp: false },
                                    "HellO World");
        assert!(!matcher("ello wo"));

        assert!(matcher("hello world"));
        assert!(matcher("prefix hEllo worLd postfix"));
    }

    #[test]
    fn matcher_regexp() {
        let matcher = build_matcher(&Grep { filename: String::new(), query: String::new(), ignore_case: false, is_regexp: true },
                                    r"He.*\sWorld");
        assert!(!matcher("ello wo"));
        assert!(!matcher("hello world"));

        assert!(matcher("Hello World"));
        assert!(matcher("prefix Hello World postfix"));
    }

    #[test]
    fn matcher_regexp_ignore_case() {
        let matcher = build_matcher(&Grep { filename: String::new(), query: String::new(), ignore_case: true, is_regexp: true },
                                    r"He.*\sWorld");
        assert!(!matcher("ello wo"));

        assert!(matcher("hello world"));
        assert!(matcher("Hello World"));
        assert!(matcher("prefix Hello World postfix"));
    }
}
