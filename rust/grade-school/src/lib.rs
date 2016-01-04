use std::collections::HashMap;

pub struct School {
    grades: HashMap<i32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }
    pub fn add(&mut self, _grade: i32, _name: &str) {
        let entry = self.grades.entry(_grade).or_insert(vec![]);
        entry.push(_name.to_owned());
    }
    pub fn grades(&self) -> Vec<i32> {
        self.grades.keys().map(|&x| x).collect::<Vec<i32>>()
    }
    pub fn grade(&self, _grade: i32) -> Option<&Vec<String>> {
        None
    }
}
