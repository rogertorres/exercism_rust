use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();

    for (points, letters) in h.iter(){
        for l in letters{
            result.insert(l.to_ascii_lowercase(),*points);
        }
    };

    result
}