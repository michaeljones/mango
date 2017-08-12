
use yaml_rust::Yaml;

use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use Node;
use NodeRef;
use NodeBuilder;
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

pub struct ToIntBuilder {}

impl NodeBuilder for ToIntBuilder {
    fn build(&self, id: i64, name: &str, _entry: &Yaml) -> Option<NodeRef> {
        if name == "to-int" {
            return Some(Rc::new(RefCell::new(ToInt {
                id: id,
                input: None,
            })));
        }
        None
    }
}
