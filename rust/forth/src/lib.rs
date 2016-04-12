use std::io::prelude::*;
use std::fmt;
use std::str::FromStr;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<i32>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "{:?}", *self),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Error {
        Error::InvalidWord
    }
}

fn join<T>(v: &[T], glue: &str) -> String
    where T: fmt::Display
{
    v.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(glue)
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: vec![] }
    }

    pub fn format_stack(&self) -> String {
        join(&self.stack, " ")
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let not_a_word = |c: char| {
            !(c.is_numeric() || "+-/*".contains(c))
        };
        for word in input.split(not_a_word) {
            let n = try!(i32::from_str(word));
            self.stack.push(n);
        }
        Ok(())
    }
}
