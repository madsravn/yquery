use std::collections::HashMap;

pub fn contains(smaller: &HashMap<String, String>, bigger: &HashMap<String, String>) -> bool {
    smaller
        .iter()
        .all(|(k, v)| bigger.get(k).filter(|v2| v == *v2).is_some())
}

pub fn contains_keys(keys: &Vec<String>, map: &HashMap<String, String>) -> bool {
    keys.iter().all(|v| map.get(v).is_some())
}
