
extern crate json;

use yaml_rust::Yaml;

use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use Node;
use NodeRef;
use NodeBuilder;
use FlowData;

pub struct JsonStringify {
    pub id: i64,
    pub input: Option<Rc<RefCell<Node>>>,
}

impl Node for JsonStringify {
    fn id(&self) -> i64 {
        self.id
    }

    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                    FlowData::Json(data) => FlowData::String(json::stringify(data)),
                    FlowData::Error(string) => FlowData::Error(string),
                    _ => FlowData::Error("Unknown data".to_string()),
                };
            }
        }
    }

    fn set_input(&mut self, node: Option<Rc<RefCell<Node>>>, _index: Option<i64>) -> () {
        self.input = node;
    }

    fn get_spec(&self) -> Spec {
        Spec {
            id: self.id,
            type_: String::from("json-stringify"),
            attributes: vec![],
        }
    }
}

pub struct JsonStringifyBuilder {}

impl NodeBuilder for JsonStringifyBuilder {
    fn build(&self, id: i64, name: &str, _entry: &Yaml) -> Option<NodeRef> {
        if name == "json-stringify" {
            return Some(Rc::new(RefCell::new(JsonStringify {
                id: id,
                input: None,
            })));
        }
        None
    }
}
