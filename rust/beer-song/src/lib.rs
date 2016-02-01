fn plural(word: String, n: usize) -> String {
    if n == 0 {
        return "no more bottles".to_owned();
    }
    let suffix = if n == 1 {
        ""
    } else {
        "s"
    };
    format!("{} {}{}", n, word, suffix)
}

pub fn verse(nbottles: usize) -> String {
    let bottle_plural = |n| plural("bottle".to_owned(), n);
    let how_much = |n| {
        if n == 1 {
            "it"
        } else {
            "one"
        }
    };
    match nbottles {
        0 => {
            "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy \
             some more, 99 bottles of beer on the wall.\n"
                .to_owned()
        }
        _ => {
            format!("{} of beer on the wall, {} of beer.
Take {} down and pass it around, {} of \
                     beer on the wall.\n",
                    bottle_plural(nbottles),
                    bottle_plural(nbottles),
                    how_much(nbottles),
                    bottle_plural(nbottles - 1))
        }
    }
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
