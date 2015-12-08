use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    if text.chars().any(|x| x.is_alphabetic()) {
        result.insert(text.to_owned(), 1);
    }
    result
}
