
extern crate yaml_rust;
extern crate json;

use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::DerefMut;

mod nodes;

use nodes::standard_in::StandardIn;
use nodes::standard_out::StandardOut;
use nodes::lines::Lines;
use nodes::json_parse::JsonParse;
use nodes::json_keys::JsonKeys;
use nodes::to_int::ToInt;
use nodes::sum::Sum;
use nodes::string_contains::StringContains;

#[derive(Debug)]
pub enum FlowData {
    Error(String),
    String(String),
    StringArray(Vec<String>),
    Int(i64),
    IntArray(Vec<i64>),
    Json(json::JsonValue),
}

pub trait Node {
    fn id(&self) -> i64;
    fn pull(&mut self) -> FlowData;
    fn set_input(&mut self, Rc<RefCell<Node>>) -> ();
}

fn pull(node: &mut Node) -> FlowData {
    node.pull()
}

fn build(entry: &Yaml) -> Option<Rc<RefCell<Node>>> {
    match (entry["id"].as_i64(), entry["type"].as_str()) {
        (Some(id), Some("standard-in")) => {
            return Some(Rc::new(RefCell::new(StandardIn { id: id })));
        }
        (Some(id), Some("standard-out")) => {
            return Some(Rc::new(RefCell::new(StandardOut {
                                                 id: id,
                                                 input: None,
                                             })));
        }
        (Some(id), Some("lines")) => {
            return Some(Rc::new(RefCell::new(Lines {
                                                 id: id,
                                                 input: None,
                                             })));
        }
        (Some(id), Some("json-parse")) => {
            return Some(Rc::new(RefCell::new(JsonParse {
                                                 id: id,
                                                 input: None,
                                             })));
        }
        (Some(id), Some("json-keys")) => {
            return Some(Rc::new(RefCell::new(JsonKeys {
                                                 id: id,
                                                 input: None,
                                             })));
        }
        (Some(id), Some("to-int")) => {
            return Some(Rc::new(RefCell::new(ToInt {
                                                 id: id,
                                                 input: None,
                                             })));
        }
        (Some(id), Some("sum")) => {
            return Some(Rc::new(RefCell::new(Sum {
                                                 id: id,
                                                 input: None,
                                             })));
        }
        (Some(id), Some("string-contains")) => {
            if let Some(value) = entry["value"].as_str() {
                return Some(Rc::new(RefCell::new(StringContains {
                                                     id: id,
                                                     input: None,
                                                     value: value.to_string(),
                                                 })));
            } else {
                return None;
            }
        }
        _ => return None,
    }
}

fn connect(from: i64, to: i64, node_map: &HashMap<i64, Rc<RefCell<Node>>>) -> () {
    match (node_map.get(&from), node_map.get(&to)) {
        (Some(from_node), Some(to_node)) => to_node.borrow_mut().set_input(from_node.clone()),
        _ => println!("Unable to find nodes matching ids: {:?} & {:?}", from, to),
    }
}

fn main() {

    let mut file = File::open("example.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();

    let yaml_nodes = docs[0]["nodes"].as_vec();

    let mut built_nodes = vec![];
    let mut node_map = HashMap::new();

    match yaml_nodes {
        Some(ref entries) => {
            for entry in entries.iter() {
                if let Some(node) = build(entry) {
                    built_nodes.push(node.clone());
                    node_map.insert(node.borrow_mut().id(), node.clone());
                    println!("Building {:?}", entry);
                } else {
                    println!("Failed to build {:?}", entry)
                }
            }
        }
        None => println!("No nodes in Yaml"),
    }

    let yaml_connections = docs[0]["connections"].as_vec();

    let mut end_node_id = 1;

    match yaml_connections {
        Some(ref connections) => {
            for connection in connections.iter() {
                match (connection["from"].as_i64(), connection["to"].as_i64()) {
                    (Some(from), Some(to)) => {
                        connect(from, to, &node_map);
                        if end_node_id == from {
                            end_node_id = to;
                        }
                    }
                    _ => println!("Failed to read connection information"),
                }
            }
        }
        None => println!("No connections"),
    }

    if let Some(node) = node_map.get(&end_node_id) {
        pull(node.borrow_mut().deref_mut());
    } else {
        println!("Unable to find end node");
    }
}
