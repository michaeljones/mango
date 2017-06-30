
extern crate json;

use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use Node;
use FlowData;

pub struct JsonObject {
    pub id: i64,
    pub keys_input: Option<Rc<RefCell<Node>>>,
    pub values_input: Option<Rc<RefCell<Node>>>,
}

impl Node for JsonObject {
    fn id(&self) -> i64 {
        self.id
    }

    fn pull(&mut self) -> FlowData {
        match (self.keys_input.clone(), self.values_input.clone()) {
            (Some(ref mut keys_input), Some(ref mut values_input)) => {
                let keys_content = keys_input.borrow_mut().pull();
                let values_content = values_input.borrow_mut().pull();

                println!("{:?}", keys_content);
                println!("{:?}", values_content);

                return match (keys_content, values_content) {
                           (FlowData::StringArray(keys), FlowData::StringArray(values)) => {
                               let mut object = json::object::Object::new();
                               for (key, value) in keys.iter().zip(values.iter()) {
                                   object.insert(key, json::JsonValue::String(value.to_string()));
                               }
                               return FlowData::Json(json::JsonValue::Object(object));
                           }
                           _ => FlowData::Error("Incorrect inputs".to_string()),
                       };
            }
            _ => FlowData::Error("Insufficient inputs".to_string()),
        }
    }

    fn set_input(&mut self, node: Option<Rc<RefCell<Node>>>, index: Option<i64>) -> () {
        match index {
            Some(1) => {
                self.keys_input = node;
            }
            Some(2) => {
                self.values_input = node;
            }
            Some(_) => println!("Invalid input index for json-object"),
            None => println!("Missing input index for json-object"),
        }
    }

    fn get_spec(&self) -> Spec {
        Spec {
            id: self.id,
            type_: String::from("json-object"),
            attributes: vec![],
        }
    }
}
