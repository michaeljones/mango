
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
                match content {
                    FlowData::StringArray(lines) => {
                        for line in lines.iter() {
                            println!("{}", line)
                        }
                    }
                    FlowData::String(text) => println!("{}", text),
                    other => {
                        println!("{:?}", other);
                    }
                }
                return FlowData::None;
            }
        }
    }

    fn set_input(&mut self, node: Rc<RefCell<Node>>) -> () {
        self.input = Some(node);
    }
}
