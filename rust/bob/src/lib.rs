pub fn reply(msg: &'static str) -> &'static str {
    let msg = msg.trim();

    if msg.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        "Whoa, chill out!"
    } else if msg.chars().last() == Some('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
