fn extract_number(digits: Vec<&str>) -> Option<String> {
    match digits.len() {
        10 => Some(digits.join("")),
        11 => {
            if Some('1') == digits[0].chars().next() {
                Some(digits[1..].join(""))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn number(s: &str) -> Option<String> {
    let digits: Vec<&str> = s.matches(char::is_numeric).collect();
    extract_number(digits)
}

pub fn area_code(_s: &str) -> Option<String> {
    None
}

pub fn pretty_print(_s: &str) -> &str {
    _s
}
