
use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use Node;
use FlowData;

pub struct ToInt {
    pub id: i64,
    pub input: Option<Rc<RefCell<Node>>>,
}

impl Node for ToInt {
    fn id(&self) -> i64 {
        self.id
    }

    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                    FlowData::StringArray(lines) => {
                        let mut output = vec![];
                        for line in &lines {
                            match line.parse::<i64>() {
                                Ok(int) => output.push(int),
                                Err(_e) => (),
                            }
                        }
                        return FlowData::IntArray(output);
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
            type_: String::from("sum"),
            attributes: vec![],
        }
    }
}
