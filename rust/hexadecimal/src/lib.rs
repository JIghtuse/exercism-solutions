pub fn hex_to_int(s: &str) -> Option<usize> {
    let is_hex = |c: char| c.is_numeric() || c as u8 >= b'a' && c as u8 <= b'f';
    if !s.chars().all(is_hex) {
        return None;
    }
    let byte_to_usize = |c| {
        if (c as char).is_numeric() {
            (c - b'0') as usize
        } else {
            (c - b'a' + 10) as usize
        }
    };
    let base : usize = 16;
    let power16 = |(pow, num): (usize, usize)| base.pow(pow as u32) * num;
    let number = s.bytes()
                  .map(byte_to_usize)
                  .rev()
                  .enumerate()
                  .map(power16)
                  .fold(0usize, |acc, digit| acc + digit);
    Some(number)
}
