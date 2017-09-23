
use yaml_rust::Yaml;

use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use Node;
use NodeRef;
use NodeBuilder;
use FlowData;

pub struct Lines {
    pub id: i64,
    pub input: Option<Rc<RefCell<Node>>>,
}

impl Node for Lines {
    fn id(&self) -> i64 {
        self.id
    }
    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                    FlowData::String(text) => {
                        let mut output = vec![];
                        for i in text.lines() {
                            output.push(i.to_string());
                        }
                        return FlowData::StringArray(output);
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
            type_: String::from("lines"),
            attributes: vec![],
        }
    }
}

pub struct LinesBuilder {}

impl NodeBuilder for LinesBuilder {
    fn build(&self, id: i64, name: &str, _entry: &Yaml) -> Option<NodeRef> {
        if name == "lines" {
            return Some(Rc::new(RefCell::new(Lines {
                id: id,
                input: None,
            })));
        }
        None
    }
}
