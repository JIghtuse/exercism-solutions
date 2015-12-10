pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    match a.len() == b.len() {
        true => {
            Ok(a.chars()
                .zip(b.chars())
                .filter(|x| x.0 != x.1)
                .count())
        }
        false => Err("inputs of different length"),
    }
}
