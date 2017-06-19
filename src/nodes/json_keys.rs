
use std::rc::Rc;
use std::cell::RefCell;

use Node;
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
}
