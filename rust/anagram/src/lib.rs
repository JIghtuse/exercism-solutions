fn is_anagram(a: &str, b: &str) -> bool {
    let mut chars_a : Vec<char> = a.to_lowercase().chars().collect();
    chars_a.sort();

    let mut chars_b : Vec<char> = b.to_lowercase().chars().collect();
    chars_b.sort();

    *chars_a == *chars_b && a != b
}

pub fn anagrams_for<'a>(word: &str, inputs: &'a [&'a str]) -> Vec<&'a str> {
    inputs.into_iter()
          .filter(|x| is_anagram(word, x))
          .map(|x| *x)
          .collect::<Vec<&str>>()
}
