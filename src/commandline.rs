
use params::Params;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml::Yaml;
use yaml_rust::emitter::YamlEmitter;
use std::io::Write;

use SpecAttribute;


struct Save {
}

impl Save {

    pub fn run(components: &Vec<&str>, params: &mut Params) -> bool {

        if components.len() == 1 {
            return false;
        }

        let nodes = params
            .node_map
            .values()
            .map(|node| {
                let n = node.borrow();
                let spec = n.get_spec();
                let mut hash = BTreeMap::new();
                hash.insert(Yaml::String(String::from("id")), Yaml::Integer(spec.id));
                hash.insert(Yaml::String(String::from("type")),
                            Yaml::String(String::from(spec.type_)));
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
                from_hash.insert(Yaml::String(String::from("node")),
                                 Yaml::Integer(connection.from));

                let mut to_hash = BTreeMap::new();
                to_hash.insert(Yaml::String(String::from("node")),
                               Yaml::Integer(connection.to));

                let mut hash = BTreeMap::new();
                hash.insert(Yaml::String(String::from("from")), Yaml::Hash(from_hash));
                hash.insert(Yaml::String(String::from("to")), Yaml::Hash(to_hash));
                Yaml::Hash(hash)
            })
            .collect();

        let mut doc_hash = BTreeMap::new();

        doc_hash.insert(Yaml::String(String::from("nodes")), Yaml::Array(nodes));
        doc_hash.insert(Yaml::String(String::from("connections")),
                        Yaml::Array(connections));

        let mut buffer = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut buffer);
            emitter.dump(&Yaml::Hash(doc_hash));
        }

        let mut file = File::create(components[1]).unwrap();
        file.write_all(buffer.as_bytes());
        return true;
    }
}


pub fn run(text: &String, params: &mut Params) -> bool {

    let components: Vec<&str> = text.split_whitespace().collect();
    if components.len() > 1 {
        if components.len() > 1 && components[0] == "w" {
            return Save::run(&components, params);
        }
    }

    false
}
