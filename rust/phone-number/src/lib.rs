pub fn number(s: &str) -> Option<String> {
    let numbers: Vec<&str> = s.matches(char::is_numeric).collect();
    Some(numbers.join(""))
}

pub fn area_code(_s: &str) -> Option<String> {
    None
}

pub fn pretty_print(_s: &str) -> &str {
    _s
}
