use std::io::prelude::*;
use std::fmt;
use std::str::FromStr;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
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

    fn pop_value(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn top(&mut self) -> Result<Value, Error> {
        self.stack.last().cloned().ok_or(Error::StackUnderflow)
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let not_a_word = |c: char| !(c.is_alphanumeric() || "+-/*".contains(c));
        for word in input.split(not_a_word) {
            if let Ok(n) = Value::from_str(word) {
                self.stack.push(n);
            } else {
                match word.to_lowercase().as_str() {
                    "+" => {
                        let b = try!(self.pop_value());
                        let a = try!(self.pop_value());
                        self.stack.push(a + b);
                    }
                    "-" => {
                        let b = try!(self.pop_value());
                        let a = try!(self.pop_value());
                        self.stack.push(a - b);
                    }
                    "*" => {
                        let b = try!(self.pop_value());
                        let a = try!(self.pop_value());
                        self.stack.push(a * b);
                    }
                    "/" => {
                        let b = try!(self.pop_value());
                        let a = try!(self.pop_value());
                        if b == 0 {
                            return Err(Error::DivisionByZero);
                        } else {
                            self.stack.push(a / b);
                        }
                    }
                    "dup" => {
                        let a = try!(self.top());
                        self.stack.push(a);
                    }
                    "drop" => {
                        try!(self.pop_value());
                    }
                    "swap" => {
                        let b = try!(self.pop_value());
                        let a = try!(self.pop_value());
                        self.stack.push(b);
                        self.stack.push(a);
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }
}
