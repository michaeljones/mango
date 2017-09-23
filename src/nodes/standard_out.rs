
use yaml_rust::Yaml;

use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use Node;
use NodeRef;
use NodeBuilder;
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

    fn set_input(&mut self, node: Option<Rc<RefCell<Node>>>, _index: Option<i64>) -> () {
        self.input = node;
    }

    fn get_spec(&self) -> Spec {
        Spec {
            id: self.id,
            type_: String::from("standard-out"),
            attributes: vec![],
        }
    }
}

pub struct StandardOutBuilder {}

impl NodeBuilder for StandardOutBuilder {
    fn build(&self, id: i64, name: &str, _entry: &Yaml) -> Option<NodeRef> {
        if name == "standard-out" {
            return Some(Rc::new(RefCell::new(StandardOut {
                id: id,
                input: None,
            })));
        }
        None
    }
}
