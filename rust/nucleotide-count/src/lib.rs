use std::collections::HashMap;

pub fn count<S>(_: char, _: S) -> usize
    where S: Into<String>
{
    0
}

pub fn nucleotide_counts<S>(_: S) -> HashMap<char, usize>
    where S: Into<String>
{
    HashMap::new()
}
