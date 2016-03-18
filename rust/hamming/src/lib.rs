pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    if a.len() != b.len() {
        return Err("inputs of different length");
    }

    Ok(a.chars()
        .zip(b.chars())
        .filter(|&(i, j)| i != j)
        .count())
}
