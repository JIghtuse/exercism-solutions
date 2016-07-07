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
    Div
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
        let op1: i64 = try!(res);
        let operation : Operation = try!(split[pos].parse().map_err(Error::from));
        let next_op_pos = match operation {
            Operation::Div|Operation::Mul => pos + 2,
            _ => pos + 1,
        };
        let op2 : i64 = try!(split[next_op_pos].trim_right_matches('?').parse());
        match operation {
            Operation::Add => Ok(op1 + op2),
            Operation::Sub => Ok(op1 - op2),
            Operation::Div => Ok(op1 / op2),
            Operation::Mul => Ok(op1 * op2),
        }
    }
}
