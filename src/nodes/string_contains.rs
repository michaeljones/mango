
use std::rc::Rc;
use std::cell::RefCell;

use Node;
use NodeUI;
use StringFieldData;
use FlowData;

pub struct StringContains {
    pub id: i64,
    pub input: Option<Rc<RefCell<Node>>>,
    pub value: String,
}

impl Node for StringContains {
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
                               for i in &lines {
                                   if i.contains(self.value.as_str()) {
                                       output.push(i.to_string());
                                   }
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

    fn get_ui(&self) -> NodeUI {
        NodeUI::StringField(StringFieldData {
                                label: String::from("Value"),
                                field: String::from("value"),
                            })
    }
}
