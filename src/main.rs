
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
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::DerefMut;

use params::{Params, CreateState, CommandLine, InteractionMode};

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

    let mut params = Params {
        node_id: 0,
        display_menu: CreateState::None,
        mouse_x: 0.0,
        mouse_y: 0.0,
        tab_x: 0.0,
        tab_y: 0.0,
        name_input: String::new(),
        gui_nodes: HashMap::new(),
        last_node: None,
        connect_node: None,
        node_map: HashMap::new(),
        current_connection: None,
        connections: HashMap::new(),
        selected_nodes: vec![],
        command_line: CommandLine::None,
        interaction_mode: InteractionMode::Normal,
    };

    let args_count = std::env::args().count();
    if args_count == 2 {
        if let Some(filename) = std::env::args().nth(1) {
            let mut file = File::open(filename).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();

            let mut node_ids = vec![];

            let yaml_nodes = docs[0]["nodes"].as_vec();
            match yaml_nodes {
                Some(ref entries) => {
                    for entry in entries.iter() {
                        if let Some(node) = build(entry) {
                            let n = node.borrow();
                            node_ids.push(n.id());
                            params.node_map.insert(n.id(), node.clone());
                        } else {
                            println!("Failed to build {:?}", entry)
                        }
                    }
                }
                None => println!("No nodes in Yaml"),
            }

            let yaml_connections = docs[0]["connections"].as_vec();

            let mut node_connections = vec![];

            match yaml_connections {
                Some(ref connections) => {
                    for connection in connections.iter() {
                        match (
                            connection["from"]["node"].as_i64(),
                            connection["to"]["node"].as_i64(),
                        ) {
                            (Some(from), Some(to)) => {
                                build::connect(from, None, to, Some(1), &params.node_map);
                                node_connections.push((from, to));
                            }
                            _ => println!("Failed to read connection information"),
                        }
                    }
                }
                None => println!("No connections"),
            }

            let mut end_nodes = HashSet::new();

            for node_id in node_ids {
                let mut repeat = true;
                let mut id = node_id;
                while repeat {
                    repeat = false;
                    for &(from, to) in node_connections.iter() {
                        if from == id {
                            id = to;
                            repeat = true;
                        }
                    }
                }
                end_nodes.insert(id);
            }

            for node_id in end_nodes {
                if let Some(node) = params.node_map.get(&node_id) {
                    build::pull(node.borrow_mut().deref_mut());
                }
                break;
            }
        }
    } else if args_count == 1 {
        gui::gui(&mut params);
    } else {
        println!("Unexpected argument count: {:?}", args_count);
    }
}
