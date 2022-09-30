mod lib;

extern crate glob;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() {
    let grep = lib::Grep::construct_grep_from_args();
    let matcher = lib::build_matcher(&grep, grep.query.as_str());

    for path in lib::get_files(grep.filename.as_str()) {
        println!("reading file: {:?}", path);
        println!("------------");
        search_in_file(path, &matcher).unwrap();
        println!("------------");
    }
}

fn search_in_file(path: PathBuf, matcher: &dyn Fn(&str) -> bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    for line in lib::search(contents.as_str(), matcher)? {
        println!("{}", line);
    }
    Ok(())
}
