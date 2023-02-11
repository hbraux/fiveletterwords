use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Deref;
use std::path::Path;
use std::time::Instant;

static LETTERS: &[u8] = "etaonrishdlfcmugypwbvkjxzq".as_bytes();

fn main() {
    let words: Vec<String> = read_lines("../words.txt").expect("cannot read file")
        .into_iter().filter(|w| !has_duplicate(w)).collect();
    let mut pwords: Vec<&[u8]> = Vec::new();
    for w in words.iter() {
        pwords.push(w.as_bytes());
    }
    let before = Instant::now();
    let res = process("".as_bytes(), pwords);
    println!("Result: {:?} in {:.2?} s", res, before.elapsed());
}

fn process(current: &[u8], words: Vec<&[u8]>) -> String {
    if current.len() >= 25 {
        return String::from_utf8(Vec::from(current)).unwrap();
    }
    let letter = LETTERS.into_iter().find(|b| !has_letters(current, b)).unwrap();
    let (first, second): (Vec<_>, Vec<_>) = words.into_iter().partition(|w| w.contains(letter));
    for word in first {
        let mut filtered: Vec<&[u8]> = Vec::new();
        for w in second.iter() {
            if !share_letters(w, word) {
                filtered.push(w)
            }
        }
        let found = process([current, word].concat().deref(), filtered);
        if !found.is_empty() {
            return found;
        }
    }
    return "".to_string();
}


fn share_letters(word: &[u8], other: &[u8]) -> bool {
    word.into_iter().any(|c| has_letters(other,c))
}

fn has_letters(word: &[u8], letter: &u8) -> bool  {
    (1..word.len()).any(|i| word[i] == *letter)
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn has_duplicate(word: &str) -> bool {
    let bytes = word.as_bytes();
    (1..bytes.len()).any(|i| bytes[i..].contains(&bytes[i - 1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_duplicate() {
        assert_eq!(false, has_duplicate("abcde"));
        assert_eq!(true, has_duplicate("abccd"));
        assert_eq!(true, has_duplicate("abcda"));
    }


}