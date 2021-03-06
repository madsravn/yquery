use crate::parse_grammar::FieldValueStruct;
use std::collections::HashMap;
use yaml_rust::yaml;

#[derive(Clone, Debug)]
pub struct NamedDocument {
    pub name: String,
    pub doc: yaml::Yaml,
}

fn field_values_contain_key(field_values: &Vec<FieldValueStruct>, key: String) -> bool {
    field_values
        .iter()
        .filter(|x| x.parent_found == true && x.child == key)
        .count()
        > 0
}

fn update_found_parent(parent: &String, key: &String, found_parent: bool) -> bool {
    if found_parent {
        true
    } else {
        let parent_found = parent == key;
        parent_found
    }
}

fn update_looking_for(field_values: &Vec<FieldValueStruct>, key: &String) -> Vec<FieldValueStruct> {
    // TODO: A lot of 'to_string()'. Will that have an effect on performance?
    let updated = field_values
        .iter()
        .map(|x| FieldValueStruct {
            parent: x.parent.to_string(),
            child: x.child.to_string(),
            parent_found: update_found_parent(&x.parent, key, x.parent_found),
        })
        .collect();
    updated
}

pub fn find_hashmapped_values(doc: &yaml::Yaml) -> HashMap<String, Vec<String>> {
    let mut found: HashMap<String, Vec<String>> = HashMap::new();
    if let yaml::Yaml::Hash(ref h) = doc {
        for (k, v) in h.iter() {
            found.insert(string_value(k)[0].to_string(), string_value(v));
        }
    }
    found
}

// TODO: Indent level
pub fn pretty_print(doc: &yaml::Yaml) -> String {
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

pub fn string_value(doc: &yaml::Yaml) -> Vec<String> {
    match doc {
        yaml::Yaml::Array(ref v) => {
            let mut vec = Vec::new();
            for e in v {
                let values = string_value(e);
                for value in values {
                    vec.push(value);
                }
            }
            vec
        }
        yaml::Yaml::Hash(ref _h) => vec![String::from("Hash")],
        yaml::Yaml::String(ref s) => vec![s.to_string()],
        yaml::Yaml::Integer(ref i) => vec![i.to_string()],
        yaml::Yaml::Real(ref r) => vec![r.to_string()],
        yaml::Yaml::Boolean(ref b) => vec![b.to_string()],
        yaml::Yaml::Alias(ref a) => vec![a.to_string()],
        yaml::Yaml::Null => vec![String::from("Null")],
        yaml::Yaml::BadValue => vec![String::from("BadValue")],
    }
}

pub fn post_process(named_documents: &Vec<NamedDocument>) -> Vec<NamedDocument> {
    let mut vec = Vec::new();
    for document in named_documents {
        match document.doc {
            yaml::Yaml::Array(ref a) => {
                for x in a {
                    // TODO: This is ugly - and not exhaustive
                    match x {
                        yaml::Yaml::Hash(ref _h) => {
                            let new_document = NamedDocument {
                                name: document.name.clone(),
                                doc: x.clone(),
                            };
                            vec.push(new_document);
                        }
                        yaml::Yaml::String(ref _s) => {
                            let new_document = NamedDocument {
                                name: document.name.clone(),
                                doc: x.clone(),
                            };
                            vec.push(new_document);
                        }
                        yaml::Yaml::Integer(ref _i) => {
                            let new_document = NamedDocument {
                                name: document.name.clone(),
                                doc: x.clone(),
                            };
                            vec.push(new_document);
                        }
                        _ => {
                            vec.push(document.clone());
                        }
                    }
                }
            }
            _ => {
                vec.push(document.clone());
            }
        }
    }
    vec
}

// TODO: Clean up variable names!
pub fn look_for(doc: &yaml::Yaml, looking_for: &Vec<FieldValueStruct>) -> Vec<NamedDocument> {
    match *doc {
        yaml::Yaml::Array(ref v) => {
            let mut vec = Vec::new();
            for x in v {
                let arr = look_for(x, looking_for);
                for e in arr {
                    vec.push(e);
                }
            }
            vec
        }
        yaml::Yaml::Hash(ref h) => {
            let mut vec = Vec::new();
            for (k, v) in h {
                let key = k
                    .as_str()
                    .expect("Should be able to open key as string")
                    .to_string();
                if field_values_contain_key(looking_for, key.clone()) {
                    let named_document = NamedDocument {
                        name: key.to_string(),
                        doc: v.clone(),
                    };
                    vec.push(named_document);
                }
                let updated_looking_for = update_looking_for(&looking_for, &key);
                let values = look_for(v, &updated_looking_for);
                for e in values {
                    vec.push(e);
                }
            }
            vec
        }
        _ => {
            let vec = Vec::new();
            vec
        }
    }
}
