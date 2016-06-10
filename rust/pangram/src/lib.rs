use std::collections::HashSet;

const ALPHABET_LENGTH: usize = 26;

fn is_letter(c: &char) -> bool {
    let c = *c as u8;
    b'a' <= c && c <= b'z'
}

pub fn is_pangram(s: &str) -> bool {
    let letters: HashSet<char> = s.to_lowercase()
        .chars()
        .filter(is_letter)
        .collect();
    letters.len() == ALPHABET_LENGTH
}
