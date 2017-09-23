
use yaml_rust::Yaml;

use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use Node;
use NodeRef;
use NodeBuilder;
use FlowData;

pub struct JsonKeys {
    pub id: i64,
    pub input: Option<Rc<RefCell<Node>>>,
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

    fn set_input(&mut self, node: Option<Rc<RefCell<Node>>>, _index: Option<i64>) -> () {
        self.input = node;
    }

    fn get_spec(&self) -> Spec {
        Spec {
            id: self.id,
            type_: String::from("json-keys"),
            attributes: vec![],
        }
    }
}

pub struct JsonKeysBuilder {}

impl NodeBuilder for JsonKeysBuilder {
    fn build(&self, id: i64, name: &str, _entry: &Yaml) -> Option<NodeRef> {
        if name == "json-keys" {
            return Some(Rc::new(RefCell::new(JsonKeys {
                id: id,
                input: None,
            })));
        }
        None
    }
}
