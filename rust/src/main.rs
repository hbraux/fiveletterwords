use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

static LETTERS: &[u8] = "etaonrishdlfcmugypwbvkjxzq".as_bytes();

fn main() {
    let words: Vec<String> = lines_from_file("../words.txt")
        .expect("cannot load file")
        .into_iter()
        .filter(|w| has_no_duplicate(w))
        .collect();
    println!("{} words", words.len());
    let res = process("", words);
    println!("Result: {}", res);
}

fn process(current: &str, words: Vec<String>) -> &str {
    if current.len() >= 25 {
        return current;
    }
    let cbytes = current.as_bytes();
    let letter = LETTERS.into_iter().find(|b| !cbytes.contains(b)).unwrap();
    let (first, second): (Vec<_>, Vec<_>) = words.into_iter().partition(|w| contain(w, letter));
    for word in first {
        let f = second.into_iter().filter(|w| share_letters(&word, w)).collect();
        process(&(word + &current), f);
    }
    return ""
}

fn contain(word: &String, letter: &u8) -> bool {
     word.as_bytes().contains(letter)
}

fn share_letters(word: &String, other: &String) -> bool {
    other.as_bytes().into_iter().any(|c| contain(word,c))
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn has_no_duplicate(word: &str) -> bool {
    let bytes = word.as_bytes();
    (1..bytes.len()).any(|i| bytes[i..].contains(&bytes[i - 1]))
}
