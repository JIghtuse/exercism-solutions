use std::collections::HashMap;

pub struct School {
    grades: HashMap<usize, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }
    pub fn add(&mut self, _grade: usize, _name: &str) {
        let entry = self.grades.entry(_grade).or_insert(vec![]);
        entry.push(_name.to_owned());
        entry.sort();
    }
    pub fn grades(&self) -> Vec<usize> {
        let mut grades = self.grades.keys().map(|&x| x).collect::<Vec<usize>>();
        grades.sort();
        grades
    }
    pub fn grade(&self, _grade: usize) -> Option<&Vec<String>> {
        self.grades.get(&_grade)
    }
}
