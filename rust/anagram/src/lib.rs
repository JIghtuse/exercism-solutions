fn is_anagram(word_chars: &Vec<char>, input: &str) -> bool {
    let mut chars : Vec<char> = input.chars().collect();
    chars.sort();

    *word_chars == chars
}

pub fn anagrams_for<'a>(word: &str, inputs: &'a [&'a str]) -> Vec<&'a str> {
    let mut res = vec![];

    let mut word_chars : Vec<char> = word.chars().collect();
    word_chars.sort();
    let word_chars = word_chars;

    for input in inputs {
        if is_anagram(&word_chars, input) {
            res.push(*input);
        }
    }
    res
}
