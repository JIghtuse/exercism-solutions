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
                Comparison::Unequal
            }
        }
    }
}
