pub fn raindrops(number: i32) -> String {
    match number % 3 {
        0 => "Pling".to_string(),
        _ => number.to_string(),
    }
}
