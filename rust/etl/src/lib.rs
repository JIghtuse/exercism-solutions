use std::collections::BTreeMap;

pub fn transform(old: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut res: BTreeMap<String, i32> = BTreeMap::new();
    for (score, words) in old {
        for word in words {
            res.insert(word.to_lowercase(), *score);
        }
    }
    res
}
