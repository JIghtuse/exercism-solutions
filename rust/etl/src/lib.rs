use std::collections::BTreeMap;

pub fn transform(old: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut res: BTreeMap<String, i32> = BTreeMap::new();
    for i in old {
        for s in i.1 {
            res.insert(s.to_lowercase(), *i.0);
        }
    }
    res
}
