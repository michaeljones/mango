
extern crate json;

use std::rc::Rc;
use std::cell::RefCell;

use Node;
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

    fn set_input(&mut self, node: Rc<RefCell<Node>>, _index: Option<i64>) -> () {
        self.input = Some(node);
    }
}
