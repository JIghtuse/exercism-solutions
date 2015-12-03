fn is_anagram(a: &str, b: &str) -> bool {
    let a = a.to_lowercase();
    let b = b.to_lowercase();

    if a == b {
        return false; // word is not an anagram to itself; nothing to do
    }

    let sorted_chars = |s: &str| {
        let mut chars : Vec<char> = s.chars().collect();
        chars.sort();
        chars
    };

    *sorted_chars(&a) == *sorted_chars(&b)
}

pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    inputs.into_iter()
          .filter(|x| is_anagram(word, x))
          .map(|x| *x)
          .collect::<Vec<&str>>()
}
