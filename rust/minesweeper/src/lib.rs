pub fn annotate(field: &[&str]) -> Vec<String> {
    field.iter().map(|&s| s.to_owned()).collect()
}
