
use std::rc::Rc;
use std::cell::RefCell;

use Node;
use FlowData;

pub struct Sum {
    pub id: i64,
    pub input: Option<Rc<RefCell<Node>>>,
}

impl Node for Sum {
    fn id(&self) -> i64 {
        self.id
    }

    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                           FlowData::IntArray(ints) => {
                               return FlowData::Int(ints.iter().sum());
                           }
                           FlowData::Error(string) => FlowData::Error(string),
                           _ => FlowData::Error("Unknown data".to_string()),
                       };
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
}
