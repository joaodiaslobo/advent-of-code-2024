use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Utility function to safely read lines from file (https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach).
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}