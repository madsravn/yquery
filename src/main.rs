use std::collections::HashMap;
use std::env;
use std::fs;
use yaml_rust::yaml;
pub mod parse_grammar;
pub mod utility;
pub mod yaml_handler;
use parse_grammar::parse_input_specifier;
use utility::{contains_keys, contains_map};
use yaml_handler::{find_hashmapped_values, look_for, post_process, pretty_print, NamedDocument};

extern crate pest;
extern crate pest_derive;

fn specify(docs: &Vec<NamedDocument>, specifiers: &HashMap<String, String>) -> Vec<NamedDocument> {
    let mut new_docs = Vec::new();
    for doc in docs {
        let found = find_hashmapped_values(&doc.doc);
        if contains_map(specifiers, &found) {
            new_docs.push(doc.clone());
        }
    }
    new_docs
}

fn identify(docs: &Vec<NamedDocument>, ids: &Vec<String>) -> Vec<String> {
    let mut results = Vec::new();
    // TODO: Take one Vec<String> out of found and apply value to it.
    for doc in docs {
        let found = find_hashmapped_values(&doc.doc);
        if contains_keys(ids, &found) {
            let mut value = ids
                .iter()
                .map(|v| found.get(v).expect("Value should exist"))
                .flatten()
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
            results.append(&mut value);
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
        let results = post_process(&results);

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
// TODO: Implement opening multiple files instead of just one
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: Program requires two arguments. First argument is file and second argument is search query.");
    } else {
        let verbose = args.len() > 3;
        let results = run(&args[1], &args[2], verbose);
        for result in results {
            println!("{}", result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_search_query() {
        let simple_query = "service";
        let document = "documents/verify_apache.yaml";
        let result = run(document, simple_query, false);
        let assumed_result = vec![
            "service: { name: httpd, state: started}",
            "service: { name: httpd, state: restarted}",
        ];
        let matching = result
            .iter()
            .zip(&assumed_result)
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(matching, 2);
    }

    #[test]
    fn test_simple_search_query_with_id() {
        let simple_query = "service.name";
        let document = "documents/verify_apache.yaml";
        let result = run(document, simple_query, false);
        let assumed_result = vec!["httpd", "httpd"];
        let matching = result
            .iter()
            .zip(&assumed_result)
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(matching, 2);
    }
}
