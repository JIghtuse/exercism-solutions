use std::collections::HashMap;

pub fn count<S>(_: char, strand: S) -> usize
    where S: Into<String>
{
    strand.into().len()
}

pub fn nucleotide_counts<S>(_: S) -> HashMap<char, usize>
    where S: Into<String>
{
    HashMap::new()
}
