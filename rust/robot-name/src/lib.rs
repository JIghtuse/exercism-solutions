extern crate rand;
extern crate num;

use rand::distributions::{IndependentSample, Range};

#[derive(Default)]
pub struct Robot {
    name: String,
}

fn random_from_range(rng: &mut rand::ThreadRng, begin: u8, end: u8) -> u8 {
    let range = Range::new(begin, end + 1);
    range.ind_sample(rng)
}

fn random_char(rng: &mut rand::ThreadRng) -> char {
    random_from_range(rng, b'A', b'Z') as char
}

fn next_name() -> String {
    let mut rng = rand::thread_rng();
    format!("{}{}{:03}",
            random_char(&mut rng),
            random_char(&mut rng),
            random_from_range(&mut rng, 0u8, 101u8))
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: next_name() }
    }
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn reset_name(&mut self) {
        self.name = next_name();
    }
}
