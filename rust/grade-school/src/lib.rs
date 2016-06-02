use std::collections::HashMap;

#[derive(Default)]
pub struct School {
    grades: HashMap<usize, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School::default()
    }
    pub fn add(&mut self, grade: usize, name: &str) {
        let entry = self.grades.entry(grade).or_insert(vec![]);
        entry.push(name.to_owned());
        entry.sort();
    }
    pub fn grades(&self) -> Vec<usize> {
        let mut grades : Vec<usize> = self.grades.keys().cloned().collect();
        grades.sort();
        grades
    }
    pub fn grade(&self, grade: usize) -> Option<&Vec<String>> {
        self.grades.get(&grade)
    }
}
