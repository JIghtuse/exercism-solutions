#[derive(Debug,PartialEq)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist,
}

fn is_sublist<T>(haystack: &[T], needle: &[T]) -> bool
    where T: std::cmp::PartialEq
{
    needle.is_empty() || haystack.windows(needle.len()).any(|w| w == needle)
}

pub fn sublist<T>(needle: &[T], haystack: &[T]) -> Comparison
    where T: std::cmp::PartialEq
{
    if needle == haystack {
        Comparison::Equal
    } else if is_sublist(haystack, needle) {
        Comparison::Sublist
    } else if is_sublist(needle, haystack) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
