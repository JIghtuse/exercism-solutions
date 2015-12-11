pub fn hamming_distance(strand_a: &str, strand_b: &str) -> Result<usize, &'static str> {
    match strand_a.len() == strand_b.len() {
        true => {
            Ok(strand_a.chars()
                       .zip(strand_b.chars())
                       .filter(|&(a, b)| a != b)
                       .count())
        }
        false => Err("inputs of different length"),
    }
}
