extern crate regex;

use regex::Regex;
use std::collections::HashMap;

pub struct Info {
    names: HashMap<&'static str, &'static str>,
}

#[derive(Debug, PartialEq)]
pub enum NameError {
    NoName,
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Info {
    let mut names = HashMap::new();
    for pair in pairs {
        names.entry(pair.0).or_insert(pair.1);
    }
    Info { names: names }
}

impl Info {
    fn search(&self, s: &'static str) -> Option<&str> {
        if self.names.contains_key(s) {
            return Some(s);
        }

        let re = match s.chars().last().unwrap() {
            'Y' => Regex::new(s.replace("Y", "[TC]").as_ref()).unwrap(),
            _ => return None,
        };
        for key in self.names.keys() {
            if re.is_match(key) {
                return Some(key.as_ref());
            }
        }
        None
    }

    pub fn name_for(&self, s: &'static str) -> Result<&str, NameError> {
        if let Some(key) = self.search(s) {
            Ok(self.names[key].as_ref())
        } else {
            Err(NameError::NoName)
        }
    }
}
