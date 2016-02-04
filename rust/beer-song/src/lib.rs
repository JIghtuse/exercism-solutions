fn how_much(word: &str, n: usize, capitalize: bool) -> String {
    match n {
        0 => {
            if capitalize {
                format!("No more {}s", word)
            } else {
                format!("no more {}s", word)
            }
        }
        1 => format!("{} {}", n, word),
        _ => format!("{} {}s", n, word),
    }
}

pub fn verse(nbottles: usize) -> String {
    let how_much_bottles = |n, start| how_much("bottle", n, start);
    let take_what = |n| {
        if n == 1 {
            "it"
        } else {
            "one"
        }
    };
    let mut verse = format!("{} of beer on the wall, {} of beer.\n",
                            how_much_bottles(nbottles, true),
                            how_much_bottles(nbottles, false),
                            );
    if nbottles == 0 {
        verse.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else {
        verse.push_str(&format!("Take {} down and pass it around, {} of beer on the wall.\n",
                                take_what(nbottles),
                                how_much_bottles(nbottles - 1, false)));
    }
    verse
}

pub fn sing(to: usize, from: usize) -> String {
    let mut song = String::new();
    for i in (from..to + 1).rev() {
        song.push_str(&verse(i));
        if i != from {
            song.push_str("\n");
        }
    }
    song
}
