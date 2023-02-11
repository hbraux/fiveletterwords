use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

static LETTERS: &[u8] = "etaonrishdlfcmugypwbvkjxzq".as_bytes();

fn main() {
    let words: Vec<String> = read_lines("../words.txt").expect("cannot read file")
        .into_iter().filter(|w| !has_duplicate(w)).collect();
    println!("{} words", words.len());
    let res = process("", words);
    println!("Result: {}", res);
}

fn process(current: &str, words: Vec<String>) -> &str {
    if current.len() >= 20 {
        return current;
    }
    let cbytes = current.as_bytes();
    let letter = LETTERS.into_iter().find(|b| !cbytes.contains(b)).unwrap();
    let (first, second): (Vec<_>, Vec<_>) = words.into_iter().partition(|w| has_letter(w, letter));
    for word in first {
        let f = second.clone().into_iter().filter(|w| !share_letters(w, &word)).collect();
        process(&(word + &current), f);
    }
    return "";
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