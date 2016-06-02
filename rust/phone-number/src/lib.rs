fn extract_number(digits: Vec<&str>) -> Option<String> {
    match digits.len() {
        10 => Some(digits.join("")),
        11 => {
            if digits[0].starts_with('1') {
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

pub fn area_code(s: &str) -> Option<String> {
    number(s).map(|n| n.chars().take(3).collect())
}

pub fn pretty_print(s: &str) -> String {
    match number(s) {
        None => "invalid".to_owned(),
        Some(num) => {
            let (area, prefix, number) = (&num[..3], &num[3..6], &num[6..]);
            format!("({}) {}-{}", area, prefix, number)
        }
    }
}
