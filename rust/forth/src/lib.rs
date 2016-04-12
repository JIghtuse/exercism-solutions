pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth
    }

    pub fn format_stack(&self) -> String {
        String::new()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        Err(Error::InvalidWord)
    }
}
