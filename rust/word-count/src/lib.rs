use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.trim_matches(|c: char| !c.is_alphabetic());
        if !word.is_empty() {
            let counter = result.entry(word.to_owned()).or_insert(0);
            *counter += 1;
        }
    }
    result
}
