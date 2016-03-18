pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    Ok(a.chars()
        .zip(b.chars())
        .filter(|&(i, j)| i != j)
        .count())
}
