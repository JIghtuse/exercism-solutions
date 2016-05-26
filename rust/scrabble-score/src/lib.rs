#![feature(iter_arith)]

fn letter_score(c: char) -> u32 {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'd' | 'g' => 2,
        'b' | 'c' | 'm' | 'p' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q' | 'z' => 10,
        _ => unreachable!(),
    }
}

pub fn score(s: &str) -> u32 {
    s.chars().map(|c| letter_score(c.to_lowercase().next().unwrap())).sum()
}
