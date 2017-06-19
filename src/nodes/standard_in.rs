
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std;

use Node;
use FlowData;

pub struct StandardIn {
    pub id: i64,
    pub cache: Option<FlowData>,
}

impl Node for StandardIn {
    fn id(&self) -> i64 {
        self.id
    }

    fn pull(&mut self) -> FlowData {
        match self.cache.clone() {
            Some(response) => response,
            None => {
                let stdin = std::io::stdin();
                let mut stream = stdin.lock();
                let mut content = String::new();
                let response = match stream.read_to_string(&mut content) {
                    Ok(_) => FlowData::String(content),
                    Err(_) => FlowData::Error("Failed to read from stdin".to_string()),
                };
                self.cache = Some(response.clone());
                response
            }
        }
    }

    fn set_input(&mut self, _node: Option<Rc<RefCell<Node>>>, _index: Option<i64>) -> () {}
}
