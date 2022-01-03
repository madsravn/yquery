use pest::Parser;
use std::collections::HashMap;
use std::env;
use std::fs;
use yaml_rust::yaml;
pub mod utility;
pub mod yaml_handler;
use utility::{contains, contains_keys};
use yaml_handler::{find_hashmapped_values, look_for, pretty_print, NamedDocument};

extern crate pest;
#[macro_use]
extern crate pest_derive;

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

fn specify(docs: &Vec<NamedDocument>, specifiers: &HashMap<String, String>) -> Vec<NamedDocument> {
    let mut new_docs = Vec::new();
    for doc in docs {
        let found = find_hashmapped_values(&doc.doc);
        if contains(specifiers, &found) {
            new_docs.push(doc.clone());
        }
    }
    new_docs
}

fn identify(docs: &Vec<NamedDocument>, ids: &Vec<String>) -> Vec<String> {
    let mut results = Vec::new();
    for doc in docs {
        let found = find_hashmapped_values(&doc.doc);
        if contains_keys(ids, &found) {
            let value = ids
                .iter()
                .map(|v| found.get(v).expect("Value should exist").to_string())
                .collect::<Vec<String>>()
                .join(", ");
            results.push(value);
        }
    }

    results
}

fn run(path: &str, input: &str, debug: bool) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Should be able to parse file");
    let input_specifiers = parse_input_specifier(input);
    let looking_for = input_specifiers.0;
    let specifiers = input_specifiers.1;
    let ids = input_specifiers.2;

    if debug {
        println!("{:?}", looking_for);
        println!("{:?}", specifiers);
        println!("{:?}", ids);
    }

    let mut res = Vec::new();
    //TODO: Consider moving this to the yaml_handler to remove all traces of "yaml"
    let docs = yaml::YamlLoader::load_from_str(&content).expect("Should be able to parse result");
    for doc in docs {
        let results = look_for(&doc, &looking_for);
        let results = specify(&results, &specifiers);
        if ids.is_empty() {
            for result in results {
                // TODO: Consider what you print and how you print it. Would removing everything
                // but the resulting hashmap be bad?
                let output = pretty_print(&result.doc);
                let formatted_string = format!("{}: {}", result.name, output).to_string();
                res.push(formatted_string);
            }
        } else {
            let results = identify(&results, &ids);
            for result in results {
                res.push(result.to_string());
            }
        }
    }
    res
}

// TODO: Implement clap
fn main() {
    let args: Vec<_> = env::args().collect();
    let verbose = args.len() > 3;
    let results = run(&args[1], &args[2], verbose);
    for result in results {
        println!("{}", result);
    }
}

// TODO: TESTS
