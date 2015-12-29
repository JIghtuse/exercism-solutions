extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

pub struct Robot {
    name: String,
}

fn random_char(rng: &mut rand::ThreadRng) -> char {
    let range = Range::new('A' as u8, 'Z' as u8 + 1);
    range.ind_sample(rng) as char
}

fn next_name() -> String {
    let mut rng = rand::thread_rng();
    format!("{}{}{:03}",
            random_char(&mut rng),
            random_char(&mut rng),
            Range::new(0, 101).ind_sample(&mut rng))
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: next_name() }
    }
    pub fn name<'a>(&'a self) -> &'a str {
        self.name.as_ref()
    }
    pub fn reset_name(&mut self) {}
}
