pub struct School;

impl School {
    pub fn new() -> School {
        School
    }
    pub fn add(&mut self, _grade: i32, _name: &str) {
    }
    pub fn grades(&self) -> Vec<i32> {
        vec![]
    }
    pub fn grade(&self, _grade: i32) -> Option<&Vec<String>> {
        None
    }
}
