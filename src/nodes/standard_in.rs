
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
        stream.read_to_string(&mut content);
        return FlowData::String(content);
    }

    fn set_input(&mut self, _node: Rc<RefCell<Node>>) -> () {}
}
