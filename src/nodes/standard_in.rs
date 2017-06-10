
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std;

use Node;
use FlowData;

pub struct StandardIn {
    pub id: i64,
}

impl Node for StandardIn {
    fn id(&self) -> i64 {
        self.id
    }

    fn pull(&mut self) -> FlowData {
        let stdin = std::io::stdin();
        let mut stream = stdin.lock();
        let mut content = String::new();
        return match stream.read_to_string(&mut content) {
                   Ok(_) => FlowData::String(content),
                   Err(_) => FlowData::Error("Failed to read from stdin".to_string()),
               };
    }

    fn set_input(&mut self, _node: Rc<RefCell<Node>>) -> () {}
}
