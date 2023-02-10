use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let lines: Vec<String> = lines_from_file("../words.txt").expect("Could not load lines");
    let filtered: Vec<String> = lines.into_iter().filter(|x| has_no_duplicate(x)).collect();
    println!("{}", filtered.len())
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn has_no_duplicate(word: &str) -> bool {
    let bytes = word.as_bytes();
    (1..bytes.len()).any(|i| bytes[i..].contains(&bytes[i - 1]))
}
