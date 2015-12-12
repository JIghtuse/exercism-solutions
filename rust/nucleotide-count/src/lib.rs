use std::collections::HashMap;

pub fn count<S>(needle: char, strand: S) -> usize
    where S: Into<String>
{
    strand.into().chars().filter(|&c| c == needle).count()
}

pub fn nucleotide_counts<S>(_: S) -> HashMap<char, usize>
    where S: Into<String>
{
    HashMap::new()
}
