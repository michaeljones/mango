
use yaml_rust::Yaml;

use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std;

use Spec;
use Node;
use NodeRef;
use NodeBuilder;
use FlowData;

pub struct StandardIn {
    pub id: i64,
    pub cache: Option<FlowData>,
}

impl Node for StandardIn {
    fn id(&self) -> i64 {
        self.id
    }

    fn pull(&mut self) -> FlowData {
        match self.cache.clone() {
            Some(response) => response,
            None => {
                let stdin = std::io::stdin();
                let mut stream = stdin.lock();
                let mut content = String::new();
                let response = match stream.read_to_string(&mut content) {
                    Ok(_) => FlowData::String(content),
                    Err(_) => FlowData::Error("Failed to read from stdin".to_string()),
                };
                self.cache = Some(response.clone());
                response
            }
        }
    }

    fn set_input(&mut self, _node: Option<Rc<RefCell<Node>>>, _index: Option<i64>) -> () {}

    fn get_spec(&self) -> Spec {
        Spec {
            id: self.id,
            type_: String::from("standard-in"),
            attributes: vec![],
        }
    }
}

pub struct StandardInBuilder {}

impl NodeBuilder for StandardInBuilder {
    fn build(&self, id: i64, name: &str, _entry: &Yaml) -> Option<NodeRef> {
        if name == "standard-in" {
            return Some(Rc::new(RefCell::new(StandardIn {
                id: id,
                cache: None,
            })));
        }
        None
    }
}
