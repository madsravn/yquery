use std::collections::HashMap;
use pest::Parser;
use pest_derive::Parser;

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

