#[derive(Debug,PartialEq)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist,
}

fn contains<T>(haystack: &[T], needle: &[T]) -> bool
    where T: std::cmp::PartialEq
{
    let length = needle.len();
    let nwindows = haystack.len() - length + 1;
    (0..nwindows).any(|i| needle == &haystack[i..i + length])
}

pub fn sublist<T>(needle: &[T], haystack: &[T]) -> Comparison
    where T: std::cmp::PartialEq
{
    match (needle.is_empty(), haystack.is_empty()) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => {
            if needle == haystack {
                Comparison::Equal
            } else {
                if contains(haystack, needle) {
                    Comparison::Sublist
                } else {
                    Comparison::Unequal
                }
            }
        }
    }
}
