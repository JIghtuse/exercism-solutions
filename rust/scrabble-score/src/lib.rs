pub fn score(s: &str) -> u32 {
    let s = s.to_lowercase();
    if 'a' == s.chars().next().unwrap() {
        1
    } else {
        4
    }
}
