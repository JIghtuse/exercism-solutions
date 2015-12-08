use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    if text.chars().any(|x| x.is_alphabetic()) {
        for word in text.split_whitespace() {
            result.insert(word.to_owned(), 1);
        }
    }
    result
}
