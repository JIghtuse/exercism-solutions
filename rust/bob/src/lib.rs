pub fn reply(msg: &'static str) -> &'static str {
    if msg.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
