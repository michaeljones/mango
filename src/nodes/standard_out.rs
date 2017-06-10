
use std::rc::Rc;
use std::cell::RefCell;

use Node;
use FlowData;

pub struct StandardOut {
    pub id: i64,
    pub input: Option<Rc<RefCell<Node>>>,
}


impl Node for StandardOut {
    fn id(&self) -> i64 {
        self.id
    }
    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();
                println!("{:?}", content);
                return FlowData::StringArray(vec![]);
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
}
