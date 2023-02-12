use std::fs::read_to_string;
use std::ops::Deref;
use std::time::Instant;

static LETTERS: &[u8] = "etaonrishdlfcmugypwbvkjxzq".as_bytes();

// assuming ascii words
type Word = [u8];

fn main() {
    let before = Instant::now();
    let str = read_to_string("../words.txt").expect("cannot read");
    let words: Vec<&Word> = str.lines().filter(|w| !has_duplicate(w)).map(|x| x.as_bytes()).collect();
    let result = process(&[], words);
    println!("Result: {:?} found in {} ms", result.unwrap(), before.elapsed().as_millis());
}

fn process(current: &Word, words: Vec<&Word>) -> Option<String> {
    if current.len() >= 25 {
        return Some(String::from_utf8(Vec::from(current)).unwrap());
    }
    let letter = LETTERS.into_iter().find(|b| !has_letters(current, b)).unwrap();
    let (first, second): (Vec<_>, Vec<_>) = words.into_iter().partition(|w| w.contains(letter));
    for word in first {
        if let Some(x) = process(
            [current, word].concat().deref(),
            second.clone().into_iter().filter(|w| !share_letters(w, word)).collect()) {
            return Some(x);
        }
    }
    return None;
}

fn share_letters(word: &Word, other: &Word) -> bool {
    word.into_iter().any(|c| has_letters(other, c))
}

fn has_letters(word: &Word, letter: &u8) -> bool {
    (1..word.len()).any(|i| word[i] == *letter)
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
    fn test_share_letters() {
        let word = "abcdefghi".as_bytes();
        assert_eq!(false, share_letters(word, "mnopq".as_bytes()));
        assert_eq!(true, share_letters(word, "mnapq".as_bytes()));
    }
}
