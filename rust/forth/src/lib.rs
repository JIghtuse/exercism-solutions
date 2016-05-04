extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::io::prelude::*;
use std::fmt;
use std::str::FromStr;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    macros: HashMap<String, String>,
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
        Forth {
            stack: vec![],
            macros: HashMap::new(),
        }
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
        let re = Regex::new(r": ([:alpha:]+) ([^;]+);").unwrap();
        let mut macro_present = false;

        for word in re.captures_iter(input) {
            let name = word.at(1).unwrap().to_lowercase();
            self.macros.entry(name).or_insert(word.at(2).unwrap().to_string());
            macro_present = true;
        }
        if macro_present {
            return Ok(());
        }

        for word in input.split(&not_a_word) {
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
                    "over" => {
                        let b = try!(self.pop_value());
                        let a = try!(self.pop_value());
                        self.stack.push(a);
                        self.stack.push(b);
                        self.stack.push(a);
                    }
                    "" => (),
                    possible_macro => {
                        if self.macros.contains_key(possible_macro) {
                            let macros = self.macros.clone();
                            let res = self.eval(&macros[possible_macro]);
                            if res.is_err() {
                                return res;
                            }
                        } else {
                            return Err(Error::UnknownWord);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
