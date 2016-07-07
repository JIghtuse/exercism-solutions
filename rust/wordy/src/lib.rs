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
        let split: Vec<&str> = self.command.split_whitespace().collect();
        let mut pos = 0usize;

        let mut res = Err(Error::InvalidCommand);
        while pos < split.len() && res.is_err() {
            res = split[pos].parse::<i64>().map_err(Error::from);
            pos += 1;
        }

        let mut result = try!(res);

        while pos < split.len() {
            let operation = try!(split[pos].parse().map_err(Error::from));
            pos = match operation {
                Operation::Div | Operation::Mul => pos + 2,
                _ => pos + 1,
            };
            let op: i64 = try!(split[pos].trim_right_matches('?').parse());
            result = match operation {
                Operation::Add => result + op,
                Operation::Sub => result - op,
                Operation::Div => result / op,
                Operation::Mul => result * op,
            };
            pos += 1;
        }
        Ok(result)
    }
}
