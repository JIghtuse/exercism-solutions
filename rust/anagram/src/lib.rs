fn is_anagram(a: &str, b: &str) -> bool {
    let sorted_chars = |s: &str| {
        let mut chars : Vec<char> = s.to_lowercase().chars().collect();
        chars.sort();
        chars
    };

    *sorted_chars(a) == *sorted_chars(b) && a != b
}

pub fn anagrams_for<'a>(word: &str, inputs: &'a [&'a str]) -> Vec<&'a str> {
    inputs.into_iter()
          .filter(|x| is_anagram(word, x))
          .map(|x| *x)
          .collect::<Vec<&str>>()
}
