#[derive(Debug,PartialEq)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist,
}

pub fn sublist<T>(needle: &[T], haystack: &[T]) -> Comparison {
    match (needle.is_empty(), haystack.is_empty()) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
