use std::collections::HashMap;

pub fn contains_map(
    smaller: &HashMap<String, String>,
    bigger: &HashMap<String, Vec<String>>,
) -> bool {
    smaller
        .iter()
        .all(|(k, v)| bigger.get(k).filter(|v2| v.to_string() == *v2[0]).is_some())
}

pub fn contains_keys<T>(keys: &Vec<String>, map: &HashMap<String, T>) -> bool {
    keys.iter().all(|v| map.get(v).is_some())
}

pub fn contains(smaller: &HashMap<String, String>, bigger: &HashMap<String, String>) -> bool {
    smaller
        .iter()
        .all(|(k, v)| bigger.get(k).filter(|v2| v == *v2).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contains_both_ways_is_equal() {
        let map_one = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
        ]);
        let map_two = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
        ]);

        let c_test_one = contains(&map_one, &map_two);
        let c_test_two = contains(&map_two, &map_one);

        assert_eq!(c_test_one, true);
        assert_eq!(c_test_one, c_test_two);
    }

    #[test]
    fn contains_both_ways_not_equal() {
        let map_one = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
        ]);
        let map_two = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
            (String::from("baz"), String::from("3")),
        ]);
        let c_test_one = contains(&map_one, &map_two);
        let c_test_two = contains(&map_two, &map_one);
        assert_eq!(c_test_one, true);
        assert_eq!(c_test_two, false);
        assert_ne!(c_test_one, c_test_two);
    }

    #[test]
    fn contains_work() {
        let map_one = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
        ]);
        let map_two = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
            (String::from("baz"), String::from("3")),
        ]);
        let c_test = contains(&map_one, &map_two);
        assert_eq!(c_test, true);
    }

    #[test]
    fn contains_reject_one() {
        let map_one = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
        ]);
        let map_two = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
            (String::from("baz"), String::from("3")),
        ]);
        let c_test = contains(&map_two, &map_one);
        assert_eq!(c_test, false);
    }

    #[test]
    fn contains_reject_two() {
        let map_one = HashMap::from([
            (String::from("foo"), String::from("2")),
            (String::from("bar"), String::from("2")),
        ]);
        let map_two = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
            (String::from("baz"), String::from("3")),
        ]);
        let c_test = contains(&map_two, &map_one);
        assert_eq!(c_test, false);
    }

    #[test]
    fn contains_keys_work() {
        let vec = vec![String::from("foo"), String::from("bar")];
        let map = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
            (String::from("baz"), String::from("3")),
        ]);
        let c_test = contains_keys(&vec, &map);
        assert_eq!(c_test, true);
    }

    #[test]
    fn contains_keys_reject() {
        let vec = vec![String::from("foo"), String::from("zoom")];
        let map = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2")),
            (String::from("baz"), String::from("3")),
        ]);
        let c_test = contains_keys(&vec, &map);
        assert_eq!(c_test, false);
    }
}
