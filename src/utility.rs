use std::collections::HashMap;

pub fn contains(smaller: &HashMap<String, String>, bigger: &HashMap<String, String>) -> bool {
    smaller
        .iter()
        .all(|(k, v)| bigger.get(k).filter(|v2| v == *v2).is_some())
}

pub fn contains_keys(keys: &Vec<String>, map: &HashMap<String, String>) -> bool {
    keys.iter().all(|v| map.get(v).is_some())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contains_work() {
        let map_one = HashMap::from([(String::from("foo"), String::from("1")), (String::from("bar"), String::from("2"))]);
        let map_two = HashMap::from([(String::from("foo"), String::from("1")), (String::from("bar"), String::from("2")), (String::from("baz"), String::from("3"))]);
        let c_test = contains(&map_one, &map_two);
        assert_eq!(c_test, true);
    }

    #[test]
    fn contains_reject_one() {
        let map_one = HashMap::from([(String::from("foo"), String::from("1")), (String::from("bar"), String::from("2"))]);
        let map_two = HashMap::from([(String::from("foo"), String::from("1")), (String::from("bar"), String::from("2")), (String::from("baz"), String::from("3"))]);
        let c_test = contains(&map_two, &map_one);
        assert_eq!(c_test, false);
    }

    #[test]
    fn contains_reject_two() {
        let map_one = HashMap::from([(String::from("foo"), String::from("2")), (String::from("bar"), String::from("2"))]);
        let map_two = HashMap::from([(String::from("foo"), String::from("1")), (String::from("bar"), String::from("2")), (String::from("baz"), String::from("3"))]);
        let c_test = contains(&map_two, &map_one);
        assert_eq!(c_test, false);
    }

}
