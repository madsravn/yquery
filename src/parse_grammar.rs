use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "command_parser.pest"]
struct CommandParser;

#[derive(Debug)]
pub struct FieldValueStruct {
    pub parent: String,
    pub child: String,
    pub parent_found: bool,
}

pub fn parse_input_specifier(
    input: &str,
) -> (Vec<FieldValueStruct>, HashMap<String, String>, Vec<String>) {
    let complete = CommandParser::parse(Rule::complete, input)
        .expect("Should be able to parse input")
        .next()
        .expect("Should be able to find first occurence");
    let mut specifiers = HashMap::new();
    let mut id_vec = Vec::new();
    let mut value_vec: Vec<FieldValueStruct> = Vec::new();
    for inner in complete.into_inner() {
        match inner.as_rule() {
            Rule::fields => {
                let fields = inner.into_inner();
                for field in fields {
                    let field_values = field.into_inner();
                    for field_value in field_values {
                        match field_value.as_rule() {
                            Rule::value_with_parent => {
                                let field_value_with_parent = field_value.into_inner();
                                let vec: Vec<String> = field_value_with_parent
                                    .map(|x| x.as_str().to_string())
                                    .collect();
                                let field_value_struct = FieldValueStruct {
                                    parent: vec[0].to_string(),
                                    child: vec[1].to_string(),
                                    parent_found: false,
                                };
                                value_vec.push(field_value_struct);
                            }
                            Rule::value => {
                                let values = field_value.into_inner();
                                for value in values {
                                    match value.as_rule() {
                                        Rule::chars_with_numbers => {
                                            let val = value.as_str();
                                            let field_value_struct = FieldValueStruct {
                                                parent: String::from(""),
                                                child: val.to_string(),
                                                parent_found: true,
                                            };
                                            value_vec.push(field_value_struct);
                                        }
                                        Rule::separator => {}
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
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
    use crate::utility::contains;

    fn create_field_value_struct_from_string(str: &str) -> FieldValueStruct {
        FieldValueStruct {
            parent: String::from(""),
            child: str.to_string(),
            parent_found: false,
        }
    }

    fn vector_child_value_equal(one: &Vec<FieldValueStruct>, two: &Vec<FieldValueStruct>) -> bool {
        if one.len() == two.len() {
            let matching = one
                .iter()
                .zip(two)
                .filter(|&(a, b)| a.child == b.child)
                .count()
                == one.len();
            matching
        } else {
            false
        }
    }

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
        let vec_one = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];
        let vec_two = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];

        assert_eq!(vector_equal(&vec_one, &vec_two), true);
        assert_eq!(vector_equal(&vec_two, &vec_one), true);
        assert_eq!(vector_equal(&vec_two, &vec_two), true);
        assert_eq!(vector_equal(&vec_one, &vec_one), true);
    }

    #[test]
    fn test_vector_equal_not_equal() {
        let vec_one = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];
        let vec_two = vec![
            String::from("one"),
            String::from("three"),
            String::from("two"),
        ];
        let vec_three = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
        ];
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

        let found_fields = vec![create_field_value_struct_from_string("service")];
        let found_specifiers: HashMap<String, String> = HashMap::new();
        let found_ids: Vec<String> = Vec::new();

        println!("Match: {:?} with {:?}", &result.0, &found_fields);
        assert_eq!(vector_child_value_equal(&result.0, &found_fields), true);
        assert_eq!(hashmap_equal(&result.1, &found_specifiers), true);
        assert_eq!(vector_equal(&result.2, &found_ids), true);
    }

    #[test]
    fn test_less_simple_search_query() {
        let simple_query = "(service|state)";
        let result = parse_input_specifier(simple_query);

        let found_fields = vec![
            create_field_value_struct_from_string("service"),
            create_field_value_struct_from_string("state"),
        ];
        let found_specifiers: HashMap<String, String> = HashMap::new();
        let found_ids: Vec<String> = Vec::new();

        assert_eq!(vector_child_value_equal(&result.0, &found_fields), true);
        assert_eq!(hashmap_equal(&result.1, &found_specifiers), true);
        assert_eq!(vector_equal(&result.2, &found_ids), true);
    }

    #[test]
    fn test_less_simple_search_query_two() {
        let simple_query = "(service|state|expression)";
        let result = parse_input_specifier(simple_query);

        let found_fields = vec![
            create_field_value_struct_from_string("service"),
            create_field_value_struct_from_string("state"),
            create_field_value_struct_from_string("expression"),
        ];
        let found_specifiers: HashMap<String, String> = HashMap::new();
        let found_ids: Vec<String> = Vec::new();

        assert_eq!(vector_child_value_equal(&result.0, &found_fields), true);
        assert_eq!(hashmap_equal(&result.1, &found_specifiers), true);
        assert_eq!(vector_equal(&result.2, &found_ids), true);
    }
}
