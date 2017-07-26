
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

use yaml_rust::yaml::Yaml;
use yaml_rust::emitter::YamlEmitter;

use SpecAttribute;
use commands::{UndoStack, Command};
use params::Params;


struct SaveCommand {
    components: Vec<String>,
}

impl SaveCommand {
    pub fn new(components: &Vec<String>) -> Self {
        SaveCommand {
            components: components.clone(),
        }
    }

    pub fn new_ref(components: &Vec<String>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(SaveCommand::new(components)))
    }
}

impl Command for SaveCommand {
    fn is_undoable(&self) -> bool {
        false
    }

    fn execute(&mut self, params: &mut Params) {

        if self.components.len() == 1 {
            return;
        }

        let nodes = params
            .node_map
            .values()
            .map(|node| {
                let n = node.borrow();
                let spec = n.get_spec();
                let mut hash = BTreeMap::new();
                hash.insert(Yaml::String(String::from("id")), Yaml::Integer(spec.id));
                hash.insert(
                    Yaml::String(String::from("type")),
                    Yaml::String(String::from(spec.type_)),
                );
                for entry in spec.attributes {
                    match entry {
                        SpecAttribute::String(name, value) => {
                            hash.insert(Yaml::String(name), Yaml::String(value));
                        }
                        SpecAttribute::Int(name, value) => {
                            hash.insert(Yaml::String(name), Yaml::Integer(value));
                        }
                    }
                }

                Yaml::Hash(hash)
            })
            .collect();

        let connections = params
            .connections
            .values()
            .map(|connection| {

                let mut from_hash = BTreeMap::new();
                from_hash.insert(
                    Yaml::String(String::from("node")),
                    Yaml::Integer(connection.from),
                );

                let mut to_hash = BTreeMap::new();
                to_hash.insert(
                    Yaml::String(String::from("node")),
                    Yaml::Integer(connection.to),
                );

                let mut hash = BTreeMap::new();
                hash.insert(Yaml::String(String::from("from")), Yaml::Hash(from_hash));
                hash.insert(Yaml::String(String::from("to")), Yaml::Hash(to_hash));
                Yaml::Hash(hash)
            })
            .collect();

        let mut doc_hash = BTreeMap::new();

        doc_hash.insert(Yaml::String(String::from("nodes")), Yaml::Array(nodes));
        doc_hash.insert(
            Yaml::String(String::from("connections")),
            Yaml::Array(connections),
        );

        let mut buffer = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut buffer);
            emitter.dump(&Yaml::Hash(doc_hash));
        }

        let mut file = File::create(self.components[1].clone()).unwrap();
        file.write_all(buffer.as_bytes());
    }

    fn redo(&mut self, params: &mut Params) {}

    fn undo(&mut self, params: &mut Params) {}
}


pub fn run(text: &String, params: &mut Params, undo_stack: &mut UndoStack) -> bool {

    let components: Vec<String> = text.split_whitespace()
        .map(|str| String::from(str))
        .collect();
    if components.len() > 1 {
        let mut command = None;
        if components[0] == "w" {
            command = Some(SaveCommand::new_ref(&components));
        }

        if let Some(comm) = command {

            let undoable;
            {
                let mut c = comm.borrow_mut();
                c.execute(params);
                undoable = c.is_undoable();
            }

            if undoable {
                undo_stack.push(comm);
            }

            return true;
        }
    }

    false
}
