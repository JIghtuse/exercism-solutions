use std::collections::HashMap;

const DNA_NUCLEOTIDS: [char; 4] = ['G', 'C', 'T', 'A'];

pub fn count<S>(needle: char, strand: S) -> usize
    where S: Into<String>
{
    strand.into().matches(needle).count()
}

pub fn nucleotide_counts<S>(strand: S) -> HashMap<char, usize>
    where S: Into<String>
{
    let mut result: HashMap<char, usize> = HashMap::with_capacity(4);
    for nucleotide in DNA_NUCLEOTIDS.iter() {
        result.insert(*nucleotide, 0);
    }

    for c in strand.into().chars() {
        if let Some(counter) = result.get_mut(&c) {
            *counter += 1;
        } else {
            panic!("Incorrect nucleotid in input: {}", c);
        }
    }
    result
}
