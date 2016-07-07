use std::num;
use std::ops::{Add, Sub, Div, Mul};

pub struct WordProblem {
    command: String,
}

#[derive(Debug)]
pub enum Error {
    InvalidCommand,
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Self {
        Error::InvalidCommand
    }
}

pub type Operation = fn(i64, i64) -> i64;

fn parse_operation(s: &str) -> Result<Operation, Error> {
    match s {
        "plus" => Ok(Add::add),
        "minus" => Ok(Sub::sub),
        "divided" => Ok(Div::div),
        "multiplied" => Ok(Mul::mul),
        _ => Err(Error::InvalidCommand),
    }
}

impl WordProblem {
    pub fn new(command: &str) -> WordProblem {
        WordProblem { command: command.to_string() }
    }
    pub fn answer(&self) -> Result<i64, Error> {
        let mut split = self.command.split_whitespace();

        let mut first_op = Err(Error::InvalidCommand);
        while let Some(word) = split.next() {
            if let Ok(op) = word.parse::<i64>() {
                first_op = Ok(op);
                break;
            }
        }
        let mut result = try!(first_op);

        while let Some(word) = split.next() {
            let operation = try!(parse_operation(word));
            let word = split.next();
            let word = if operation == Div::div || operation == Mul::mul {
                split.next()
            } else {
                word
            };
            let word = try!(word.ok_or(Error::InvalidCommand));
            let op: i64 = try!(word.trim_right_matches('?').parse());
            result = operation(result, op);
        }
        Ok(result)
    }
}
