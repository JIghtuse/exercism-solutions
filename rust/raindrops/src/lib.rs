pub fn raindrops(number: i32) -> String {
    match (number % 3, number % 5, number % 7) {
        (0, 0, _) => "PlingPlang".to_string(),
        (0, _, _) => "Pling".to_string(),
        (_, 0, _) => "Plang".to_string(),
        (_, _, 0) => "Plong".to_string(),
        _ => number.to_string(),
    }
}
