use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

static LETTER_FREQUENCIES: &[u8] = "etaonrishdlfcmugypwbvkjxzq".as_bytes();

fn main() {
    let lines = lines_from_file("../words.txt").expect("Could not load lines");
    let filtered: Vec<String> = lines.into_iter().filter(|w| has_no_duplicate(w)).collect();
    println!("{} words", filtered.len());
    process("", filtered);

}

fn process(current: &str, words: Vec<String>) -> &str {
    if current.len() >= 25 {
        return current;
    }
    let letter = LETTER_FREQUENCIES.chars().find(|c| !chars.contains(c)).unwrap();
    let (with, wo) = words.into_iter().partition(|w| w.contains(letter));

    return ""
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn has_no_duplicate(word: &str) -> bool {
    let bytes = word.as_bytes();
    (1..bytes.len()).any(|i| bytes[i..].contains(&bytes[i - 1]))
}
