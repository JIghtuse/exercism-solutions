pub struct WordProblem {}

#[derive(Debug)]
pub enum Error {
    InvalidCommand,
}

impl WordProblem {
    pub fn new(_: &str) -> WordProblem {
        WordProblem {}
    }
    pub fn answer(&self) -> Result<i64, Error> {
        Err(Error::InvalidCommand)
    }
}
