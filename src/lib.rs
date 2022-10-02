use std::env;
use std::error::Error;
use regex::RegexBuilder;
use std::iter::{Filter, Map};
use std::path::PathBuf;
use glob::{glob, GlobResult, Paths};

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

pub fn get_paths(pattern: &str) -> Map<Paths, fn(GlobResult) -> std::path::PathBuf> {
    glob(pattern).unwrap().map(|path| path.unwrap())
}

pub fn get_files(pattern: &str) -> Filter<Map<Paths, fn(GlobResult) -> PathBuf>, fn(&PathBuf) -> bool> {
    return get_paths(pattern).filter(|path| path.is_file());
}

pub fn search<'a, F>(contents: &'a str, matcher: F)
                     -> Result<Vec<String>, Box<dyn Error>>
    where F: Fn(&str) -> bool {
    let mut matched_lines = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if matcher(line) {
            matched_lines.push(format!(":{} {}", i + 1, line));
        };
    };
    Ok(matched_lines)
}

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
        assert_eq!(search("hello\nworld", |s| s == "world").unwrap()[0], ":2 world");
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
