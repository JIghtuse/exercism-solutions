use std::num;

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
        let operator = split[pos];
        let op2: i64 = try!(split[pos + 1].trim_right_matches('?').parse());
        match operator {
            "plus" => Ok(op1 + op2),
            "minus" => Ok(op1 - op2),
            _ => Err(Error::InvalidCommand),
        }
    }
}
