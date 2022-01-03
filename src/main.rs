
use std::collections::HashMap;
use std::env;
use std::fs;
use yaml_rust::yaml;
use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "command_parser.pest"]
struct CommandParser;


#[derive(Clone)]
pub struct NamedDocument {
    pub name: String,
    pub doc: yaml::Yaml,
}

// TODO: Clean up variable names!
pub fn look_for(doc: &yaml::Yaml, looking_for: &Vec<String>) -> Vec<NamedDocument> {
    match *doc {
        yaml::Yaml::Array(ref v) => {
            let mut vec = Vec::new();
            for x in v {
                let arr = look_for(x, looking_for);
                for e in arr {
                    vec.push(e);
                }
            }
            return vec;
        }
        yaml::Yaml::Hash(ref h) => {
            let mut vec = Vec::new();
            for (k, v) in h {
                let key = k
                    .as_str()
                    .expect("Should be able to open key as string")
                    .to_string();
                if looking_for.contains(&key) {
                    let named_document = NamedDocument {
                        name: key.to_string(),
                        doc: v.clone(),
                    };
                    vec.push(named_document);
                }
                let values = look_for(v, looking_for);
                for e in values {
                    vec.push(e);
                }
            }
            return vec;
        }
        _ => {
            let vec = Vec::new();
            return vec;
        }
    }
}

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
                        match char_equal.as_rule() {
                            Rule::char_equal => {
                                if char_equal.as_str() != "," {
                                    let values: Vec<&str> =
                                        char_equal.as_str().split("=").collect();
                                    specifiers.insert(values[0].to_string(), values[1].to_string());
                                }
                            }
                            _ => {}
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



// TODO: Indent level
fn pretty_print(doc: &yaml::Yaml) -> String {
    match *doc {
        yaml::Yaml::Array(ref v) => {
            let mut s = String::new();
            for x in v {
                // TODO: This looks like garbage
                s.push_str("\n- ");
                let printed = pretty_print(x);
                s.push_str(&printed);
            }
            s
        }
        yaml::Yaml::Hash(ref h) => {
            let mut s = String::new();
            s.push_str("{ ");
            for (i, (k, v)) in h.iter().enumerate() {
                let key = pretty_print(k);
                s.push_str(&key);
                s.push_str(": ");
                let value = pretty_print(v);
                s.push_str(&value);
                if i != h.len() - 1 {
                    s.push_str(", ");
                }
            }
            s.push_str("}");
            s
        }
        yaml::Yaml::String(ref s) => s.to_string(),
        yaml::Yaml::Integer(ref i) => i.to_string(),
        yaml::Yaml::Real(ref r) => r.to_string(),
        yaml::Yaml::Boolean(ref b) => b.to_string(),
        yaml::Yaml::Alias(ref a) => a.to_string(),
        yaml::Yaml::Null => String::from("Null"),
        yaml::Yaml::BadValue => String::from("BadValue"),
    }
}

fn string_value(doc: &yaml::Yaml) -> String {
    match doc {
        yaml::Yaml::Array(ref _v) => String::from("Array"),
        yaml::Yaml::Hash(ref _h) => String::from("Hash"),
        yaml::Yaml::String(ref s) => s.to_string(),
        yaml::Yaml::Integer(ref i) => i.to_string(),
        yaml::Yaml::Real(ref r) => r.to_string(),
        yaml::Yaml::Boolean(ref b) => b.to_string(),
        yaml::Yaml::Alias(ref a) => a.to_string(),
        yaml::Yaml::Null => String::from("Null"),
        yaml::Yaml::BadValue => String::from("BadValue"),
    }
}

fn contains(smaller: &HashMap<String, String>, bigger: &HashMap<String, String>) -> bool {
    smaller
        .iter()
        .all(|(k, v)| bigger.get(k).filter(|v2| v == *v2).is_some())
}

fn contains_keys(keys: &Vec<String>, map: &HashMap<String, String>) -> bool {
    keys.iter().all(|v| map.get(v).is_some())
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

fn find_hashmapped_values(doc: &yaml::Yaml) -> HashMap<String, String> {
    let mut found = HashMap::new();
    match doc {
        yaml::Yaml::Hash(ref h) => {
            for (k, v) in h.iter() {
                found.insert(string_value(k), string_value(v));
            }
        }
        _ => {}
    }
    found
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
    let results = run(&args[1], &args[2], true);
    for result in results {
        println!("{}", result);
    }
}


// TODO: TESTS
