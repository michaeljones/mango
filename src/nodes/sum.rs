
use yaml_rust::Yaml;

use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use Node;
use NodeRef;
use NodeBuilder;
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

pub struct SumBuilder {}

impl NodeBuilder for SumBuilder {
    fn build(&self, id: i64, name: &str, _entry: &Yaml) -> Option<NodeRef> {
        if name == "sum" {
            return Some(Rc::new(RefCell::new(Sum {
                id: id,
                input: None,
            })));
        }
        None
    }
}
