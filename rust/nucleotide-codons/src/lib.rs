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
    pub fn name_for(&self, s: &'static str) -> Result<&'static str, NameError> {
        match self.names.contains_key(s) {
            true => Ok(self.names[s]),
            false => Err(NameError::NoName),
        }
    }
}
