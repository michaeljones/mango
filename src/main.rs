
extern crate json;
extern crate clap;
extern crate yaml_rust;

#[macro_use]
extern crate conrod;

use yaml_rust::{Yaml, YamlLoader};

use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;

mod nodes;
mod gui;
mod gui_node;
mod build;
mod commands;
mod commandline;
mod params;
mod widgets;

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

pub struct StringFieldData {
    pub label: String,
    pub field: String,
}

pub enum NodeUI {
    None,
    StringField(StringFieldData),
}

#[derive(Debug)]
pub enum NodeUIData {
    None,
    StringData(String),
}

pub enum SpecAttribute {
    String(String, String),
    Int(String, i64),
}

pub struct Spec {
    id: i64,
    type_: String,
    attributes: Vec<SpecAttribute>,
}

pub trait Node {
    fn id(&self) -> i64;

    fn pull(&mut self) -> FlowData;

    fn set_input(&mut self, node: Option<Rc<RefCell<Node>>>, index: Option<i64>) -> ();

    fn get_ui(&self) -> NodeUI {
        NodeUI::None
    }

    fn get_value(&self, _field: &String) -> NodeUIData {
        NodeUIData::None
    }

    fn set_value(&mut self, _field: &String, _data: NodeUIData) {}

    fn get_spec(&self) -> Spec;
}

type NodeRef = Rc<RefCell<Node>>;

pub trait NodeBuilder {
    fn build(&self, id: i64, name: &str) -> Option<NodeRef>;
}

fn build(entry: &Yaml) -> Option<NodeRef> {
    let builders: Vec<Box<NodeBuilder>> = vec![
        Box::new(nodes::StandardInBuilder {}),
        Box::new(nodes::StandardOutBuilder {}),
    ];

    match (entry["id"].as_i64(), entry["type"].as_str()) {
        (Some(id), Some(string)) => {
            for builder in builders {
                if let Some(node_ref) = builder.build(id, string) {
                    return Some(node_ref);
                }
            }
        }
        _ => return None,
    }

    None
}

fn main() {

    let args_count = std::env::args().count();
    if args_count == 1 {
        if let Some(filename) = std::env::args().nth(1) {
            let mut file = File::open("example.yaml").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();

            let yaml_nodes = docs[0]["nodes"].as_vec();
            match yaml_nodes {
                Some(ref entries) => {
                    for entry in entries.iter() {
                        if let Some(node) = build(entry) {
                            println!("Building {:?}", entry);
                        } else {
                            println!("Failed to build {:?}", entry)
                        }
                    }
                }
                None => println!("No nodes in Yaml"),
            }
        }
    } else if args_count == 0 {
        gui::gui();
    } else {
        println!("Unexpected argument count: {:?}", args_count);
    }
}
