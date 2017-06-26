
extern crate json;

use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use Node;
use FlowData;

pub struct JsonParse {
    pub id: i64,
    pub input: Option<Rc<RefCell<Node>>>,
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

    fn set_input(&mut self, node: Option<Rc<RefCell<Node>>>, _index: Option<i64>) -> () {
        self.input = node;
    }

    fn get_spec(&self) -> Spec {
        Spec {
            id: self.id,
            type_: String::from("json-parse"),
            attributes: vec![],
        }
    }
}
