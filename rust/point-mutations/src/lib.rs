pub fn hamming_distance(strand_a: &str, strand_b: &str) -> Result<usize, &'static str> {
    if strand_a.len() == strand_b.len() {
        Ok(strand_a.chars()
                   .zip(strand_b.chars())
                   .filter(|&(a, b)| a != b)
                   .count())
    } else {
        Err("inputs of different length")
    }
}
