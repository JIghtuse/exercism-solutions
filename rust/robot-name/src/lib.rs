pub struct Robot;

impl Robot {
    pub fn new() -> Robot {
        Robot
    }
    pub fn name<'a>(&'a self) -> &'a str {
        "AB123"
    }
    pub fn reset_name(&mut self) {
    }
}
