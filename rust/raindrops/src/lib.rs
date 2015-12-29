pub fn raindrops(number: i32) -> String {
    match (number % 3, number % 5) {
        (0, _) => "Pling".to_string(),
        (_, 0) => "Plang".to_string(),
        _ => number.to_string(),
    }
}
