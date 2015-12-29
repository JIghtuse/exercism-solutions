use std::collections::BTreeMap;
use std::ascii::AsciiExt;

pub fn transform(old: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut res: BTreeMap<String, i32> = BTreeMap::new();
    for i in old {
        let mut s = String::new();
        for c in i.1 {
            s.extend(c.to_lowercase().chars());
        }
        res.insert(s, *i.0);
    }
    res
}
