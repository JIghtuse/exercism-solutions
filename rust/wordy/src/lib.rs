use std::num;
use std::str::FromStr;

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

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

use Operation::*;

impl FromStr for Operation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plus" => Ok(Add),
            "minus" => Ok(Sub),
            "divided" => Ok(Div),
            "multiplied" => Ok(Mul),
            _ => Err(Error::InvalidCommand),
        }
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
            let operation = try!(word.parse().map_err(Error::from));
            let word = split.next();
            let word = match operation {
                Operation::Div | Operation::Mul => split.next(),
                _ => word,
            };
            let word = try!(word.ok_or(Error::InvalidCommand));
            let op: i64 = try!(word.trim_right_matches('?').parse());
            result = match operation {
                Operation::Add => result + op,
                Operation::Sub => result - op,
                Operation::Div => result / op,
                Operation::Mul => result * op,
            };
        }
        Ok(result)
    }
}
