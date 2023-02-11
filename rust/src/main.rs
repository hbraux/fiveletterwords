use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

static LETTERS: &[u8] = "etaonrishdlfcmugypwbvkjxzq".as_bytes();

fn main() {
    let words: Vec<String> = read_lines("../words.txt").expect("cannot read file")
        .into_iter().filter(|w| !has_duplicate(w)).collect();
    let before = Instant::now();
    let res = process("".to_string(), words);
    println!("Result: {} in {:.2?} s", res, before.elapsed());
}

fn process(current: String, words: Vec<String>) -> String {
    if current.len() >= 20 {
        return current;
    }
    let letter = LETTERS.into_iter().find(|b| !current.as_bytes().contains(b)).unwrap();
    let (first, second): (Vec<_>, Vec<_>) = words.into_iter().partition(|w| has_letter(w, letter));
    for word in first {
        let filtered: Vec<String> = second.clone().into_iter().filter(|w| !share_letters(w, &word)).collect();
        let found = process(current.clone() +  &word, filtered);
        if !found.is_empty() {
            return found;
        }
    }
    return "".to_string();
}

fn has_letter(word: &str, letter: &u8) -> bool {
    word.as_bytes().contains(letter)
}

fn share_letters(word: &str, current: &String) -> bool {
    current.as_bytes().into_iter().any(|c| has_letter(word, c))
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

    #[test]
    fn test_has_letter() {
        let letter= "z".as_bytes()[0];
        assert_eq!(false, has_letter("abcde", &letter));
        assert_eq!(true, has_letter("abzde", &letter));
    }

    #[test]
    fn test_share_letters() {
        let current = "abcdefghij".to_string();
        assert_eq!(false, share_letters("uvwxy", &current));
        assert_eq!(true, share_letters("uvaxy", &current));
    }
}