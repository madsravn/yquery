use std::collections::HashMap;
use pest::Parser;
use pest_derive::Parser;
use crate::utility::contains;

#[derive(Parser)]
#[grammar = "command_parser.pest"]
struct CommandParser;

pub fn parse_input_specifier(input: &str) -> (Vec<String>, HashMap<String, String>, Vec<String>) {
    let complete = CommandParser::parse(Rule::complete, input)
        .expect("Should be able to parse input")
        .next()
        .expect("Should be able to find first occurence");
    let mut specifiers = HashMap::new();
    let mut id_vec = Vec::new();
    let mut value_vec = Vec::new();
    for inner in complete.into_inner() {
        match inner.as_rule() {
            Rule::fields => {
                let values = inner.into_inner();
                for value in values {
                    let val = value.as_str();
                    if val != "|" {
                        value_vec.push(val.to_string());
                    }
                }
            }
            Rule::specifiers => {
                let specs = inner.into_inner();
                for spec in specs {
                    let equal_value = spec.into_inner();
                    for char_equal in equal_value {
                        if char_equal.as_rule() == Rule::char_equal {
                            if char_equal.as_str() != "," {
                                let values: Vec<&str> = char_equal.as_str().split("=").collect();
                                specifiers.insert(values[0].to_string(), values[1].to_string());
                            }
                        }
                    }
                }
            }
            Rule::ids => {
                let ids = inner.into_inner();
                for id in ids {
                    let val = id.as_str();
                    if val != "," && val != " " {
                        id_vec.push(val.to_string());
                    }
                }
            }
            _ => {}
        }
    }

    (value_vec, specifiers, id_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn vector_equal(one: &Vec<String>, two: &Vec<String>) -> bool {
        if one.len() == two.len() {
            let matching = one.iter().zip(two).filter(|&(a, b)| a == b).count() == one.len();
            matching
        } else {
            false
        }
    }

    fn hashmap_equal(one: &HashMap<String, String>, two: &HashMap<String, String>) -> bool {
        let c = contains(one, two) && contains(two, one);
        c
    }


    #[test]
    fn test_vector_equal_equal() {
        let vec_one = vec![String::from("one"), String::from("two"), String::from("three")];
        let vec_two = vec![String::from("one"), String::from("two"), String::from("three")];

        assert_eq!(vector_equal(&vec_one, &vec_two), true);
        assert_eq!(vector_equal(&vec_two, &vec_one), true);
        assert_eq!(vector_equal(&vec_two, &vec_two), true);
        assert_eq!(vector_equal(&vec_one, &vec_one), true);

    }

    #[test]
    fn test_vector_equal_not_equal() {
        let vec_one = vec![String::from("one"), String::from("two"), String::from("three")];
        let vec_two = vec![String::from("one"), String::from("three"), String::from("two")];
        let vec_three = vec![String::from("one"), String::from("two"), String::from("three"), String::from("four")];
        assert_eq!(vector_equal(&vec_one, &vec_one), true);
        assert_eq!(vector_equal(&vec_two, &vec_two), true);
        assert_eq!(vector_equal(&vec_three, &vec_three), true);
        assert_eq!(vector_equal(&vec_one, &vec_two), false);
        assert_eq!(vector_equal(&vec_two, &vec_one), false);
        assert_eq!(vector_equal(&vec_one, &vec_three), false);
        assert_eq!(vector_equal(&vec_three, &vec_one), false);
        assert_eq!(vector_equal(&vec_two, &vec_three), false);
        assert_eq!(vector_equal(&vec_three, &vec_two), false);
    }
    
    #[test]
    fn test_simple_search_query() {
        let simple_query = "service";
        let result = parse_input_specifier(simple_query);

        let found_fields = vec![String::from("service")];
        let found_specifiers: HashMap<String, String> = HashMap::new();
        let found_ids: Vec<String> = Vec::new();

        assert_eq!(vector_equal(&result.0, &found_fields), true);
        assert_eq!(hashmap_equal(&result.1, &found_specifiers), true);
        assert_eq!(vector_equal(&result.2, &found_ids), true);
    }

    #[test]
    fn test_less_simple_search_query() {
        let simple_query = "(service|state)";
        let result = parse_input_specifier(simple_query);

        let found_fields = vec![String::from("service"), String::from("state")];
        let found_specifiers: HashMap<String, String> = HashMap::new();
        let found_ids: Vec<String> = Vec::new();

        assert_eq!(vector_equal(&result.0, &found_fields), true);
        assert_eq!(hashmap_equal(&result.1, &found_specifiers), true);
        assert_eq!(vector_equal(&result.2, &found_ids), true);
    }

    #[test]
    fn test_less_simple_search_query_two() {
        let simple_query = "(service|state|expression)";
        let result = parse_input_specifier(simple_query);

        let found_fields = vec![String::from("service"), String::from("state"), String::from("expression")];
        let found_specifiers: HashMap<String, String> = HashMap::new();
        let found_ids: Vec<String> = Vec::new();

        assert_eq!(vector_equal(&result.0, &found_fields), true);
        assert_eq!(hashmap_equal(&result.1, &found_specifiers), true);
        assert_eq!(vector_equal(&result.2, &found_ids), true);
    }


}
 
