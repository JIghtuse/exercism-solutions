#[derive(Debug,PartialEq)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist,
}

pub fn sublist<T>(_: &[T], _: &[T]) -> Comparison {
    Comparison::Equal
}
