#[derive(Debug,PartialEq)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist,
}

pub fn sublist<T: std::cmp::PartialEq>(needle: &[T], haystack: &[T]) -> Comparison {
    match (needle.is_empty(), haystack.is_empty()) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => {
            if needle == haystack {
                Comparison::Equal
            } else {
                let length = needle.len();
                let nwindows = haystack.len() - length + 1;
                if (0..nwindows).any(|i| needle == &haystack[i..i + length]) {
                    Comparison::Sublist
                } else {
                    Comparison::Unequal
                }
            }
        }
    }
}
