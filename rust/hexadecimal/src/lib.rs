pub fn hex_to_int(s: &str) -> Option<usize> {
    let byte_to_usize = |c| {
        if (c as char).is_numeric() {
            (c - b'0') as usize
        } else {
            (c - b'a' + 10) as usize
        }
    };
    let number = s.bytes()
                  .map(byte_to_usize)
                  .fold(0usize, |acc, digit| acc + digit);
    Some(number)
}
