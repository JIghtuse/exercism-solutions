pub fn hamming_distance(a: &str, b: &str) -> Result<i32, &'static str> {
    Ok(a.chars()
        .zip(b.chars())
        .filter(|x| x.0 != x.1)
        .count() as i32)
}
