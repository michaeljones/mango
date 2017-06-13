
extern crate yaml_rust;
extern crate json;
extern crate clap;

#[macro_use]
extern crate conrod;

use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::DerefMut;
use clap::{Arg, App};

mod nodes;
mod gui;
mod gui_node;
mod build;

#[derive(Debug, Clone)]
pub enum FlowData {
    None,
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
    fn set_input(&mut self, node: Rc<RefCell<Node>>, index: Option<i64>) -> ();
}

fn pull(node: &mut Node) -> FlowData {
    node.pull()
}

fn main() {

    gui::feature::gui();

    /*
    let matches = App::new("slipstream")
        .version("0.1")
        .author("Michael Jones")
        .about("Node graph for text editing")
        .arg(Arg::with_name("INPUT")
                 .help("Sets the input file to use")
                 .required(true)
                 .index(1))
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();

    let yaml_nodes = docs[0]["nodes"].as_vec();

    let mut built_nodes = vec![];
    let mut node_map = HashMap::new();

    match yaml_nodes {
        Some(ref entries) => {
            for entry in entries.iter() {
                match (entry["id"].as_i64(), entry["type"].as_str()) {
                    (Some(id), Some(type_)) => {
                        if let Some(node) = build::build(id, type_.to_string()) {
                            built_nodes.push(node.clone());
                            node_map.insert(node.borrow_mut().id(), node.clone());
                        } else {
                            println!("Failed to build {:?}", entry)
                        }
                    }
                    _ => {}
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
                match (connection["from"]["node"].as_i64(), connection["to"]["node"].as_i64()) {
                    (Some(from), Some(to)) => {
                        build::connect(from,
                                       connection["from"]["input"].as_i64(),
                                       to,
                                       connection["to"]["input"].as_i64(),
                                       &node_map);
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
    */
}
