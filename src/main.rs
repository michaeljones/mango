
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

#[derive(Debug)]
enum FlowData {
    Error(String),
    String(String),
    StringArray(Vec<String>),
    Int(i64),
    IntArray(Vec<i64>),
    Json(json::JsonValue),
}

trait Node {
    fn id(&self) -> i64;
    fn pull(&mut self) -> FlowData;
    fn set_input(&mut self, Rc<RefCell<Node>>) -> ();
}

struct StandardOut {
    id: i64,
    input: Option<Rc<RefCell<Node>>>,
}


impl Node for StandardOut {
    fn id(&self) -> i64 {
        self.id
    }
    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();
                println!("{:?}", content);
                return FlowData::StringArray(vec![]);
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
}


struct Lines {
    id: i64,
    input: Option<Rc<RefCell<Node>>>,
}

impl Node for Lines {
    fn id(&self) -> i64 {
        self.id
    }
    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                           FlowData::String(text) => {
                               let mut output = vec![];
                               for i in text.lines() {
                                   output.push(i.to_string());
                               }
                               return FlowData::StringArray(output);
                           }
                           FlowData::Error(string) => FlowData::Error(string),
                           _ => FlowData::Error("Unknown data".to_string()),
                       };
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
}

struct JsonParse {
    id: i64,
    input: Option<Rc<RefCell<Node>>>,
}

impl Node for JsonParse {
    fn id(&self) -> i64 {
        self.id
    }
    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                           FlowData::String(text) => {
                               match json::parse(&text) {
                                   Ok(data) => FlowData::Json(data),
                                   Err(_e) => FlowData::Error("Failed to parse json".to_string()),
                               }
                           }
                           FlowData::Error(string) => FlowData::Error(string),
                           _ => FlowData::Error("Unknown data".to_string()),
                       };
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
}


struct JsonKeys {
    id: i64,
    input: Option<Rc<RefCell<Node>>>,
}

impl Node for JsonKeys {
    fn id(&self) -> i64 {
        self.id
    }
    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                           FlowData::Json(data) => {
                               let mut keys = vec![];
                               for (key, _value) in data.entries() {
                                   keys.push(key.to_string());
                               }
                               return FlowData::StringArray(keys);
                           }
                           FlowData::Error(string) => FlowData::Error(string),
                           _ => FlowData::Error("Unknown data".to_string()),
                       };
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
}


struct StringContains {
    id: i64,
    input: Option<Rc<RefCell<Node>>>,
    value: String,
}

impl Node for StringContains {
    fn id(&self) -> i64 {
        self.id
    }
    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                           FlowData::StringArray(lines) => {
                               let mut output = vec![];
                               for i in &lines {
                                   if i.contains(self.value.as_str()) {
                                       output.push(i.to_string());
                                   }
                               }
                               return FlowData::StringArray(output);
                           }
                           FlowData::Error(string) => FlowData::Error(string),
                           _ => FlowData::Error("Unknown data".to_string()),
                       };
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
}


struct ToInt {
    id: i64,
    input: Option<Rc<RefCell<Node>>>,
}

impl Node for ToInt {
    fn id(&self) -> i64 {
        self.id
    }

    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                           FlowData::StringArray(lines) => {
                               let mut output = vec![];
                               for line in &lines {
                                   match line.parse::<i64>() {
                                       Ok(int) => output.push(int),
                                       Err(_e) => (),
                                   }
                               }
                               return FlowData::IntArray(output);
                           }
                           FlowData::Error(string) => FlowData::Error(string),
                           _ => FlowData::Error("Unknown data".to_string()),
                       };
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
}

struct Sum {
    id: i64,
    input: Option<Rc<RefCell<Node>>>,
}

impl Node for Sum {
    fn id(&self) -> i64 {
        self.id
    }

    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                           FlowData::IntArray(ints) => {
                               return FlowData::Int(ints.iter().sum());
                           }
                           FlowData::Error(string) => FlowData::Error(string),
                           _ => FlowData::Error("Unknown data".to_string()),
                       };
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
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
