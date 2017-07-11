
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use Node;
use FlowData;

use nodes::standard_in::StandardIn;
use nodes::standard_out::StandardOut;
use nodes::lines::Lines;
use nodes::json_parse::JsonParse;
use nodes::json_stringify::JsonStringify;
use nodes::json_keys::JsonKeys;
use nodes::json_object::JsonObject;
use nodes::to_int::ToInt;
use nodes::sum::Sum;
use nodes::string_contains::StringContains;

pub fn build(id: i64, type_: String) -> Option<Rc<RefCell<Node>>> {
    match type_.as_str() {
        "standard-in" => {
            return Some(Rc::new(RefCell::new(StandardIn {
                id: id,
                cache: None,
            })));
        }
        "standard-out" => {
            return Some(Rc::new(RefCell::new(StandardOut {
                id: id,
                input: None,
            })));
        }
        "lines" => {
            return Some(Rc::new(RefCell::new(Lines {
                id: id,
                input: None,
            })));
        }
        "json-parse" => {
            return Some(Rc::new(RefCell::new(JsonParse {
                id: id,
                input: None,
            })));
        }
        "json-stringify" => {
            return Some(Rc::new(RefCell::new(JsonStringify {
                id: id,
                input: None,
            })));
        }
        "json-keys" => {
            return Some(Rc::new(RefCell::new(JsonKeys {
                id: id,
                input: None,
            })));
        }
        "json-object" => {
            return Some(Rc::new(RefCell::new(JsonObject {
                id: id,
                keys_input: None,
                values_input: None,
            })));
        }
        "to-int" => {
            return Some(Rc::new(RefCell::new(ToInt {
                id: id,
                input: None,
            })));
        }
        "sum" => {
            return Some(Rc::new(RefCell::new(Sum {
                id: id,
                input: None,
            })));
        }
        "string-contains" => {
            return Some(Rc::new(RefCell::new(StringContains {
                id: id,
                input: None,
                value: "".to_string(),
            })));
        }
        _ => return None,
    }
}

pub fn connect(
    from: i64,
    _from_input: Option<i64>,
    to: i64,
    to_input: Option<i64>,
    node_map: &HashMap<i64, Rc<RefCell<Node>>>,
) -> () {

    match (node_map.get(&from), node_map.get(&to)) {
        (Some(from_node), Some(to_node)) => {
            to_node
                .borrow_mut()
                .set_input(Some(from_node.clone()), to_input)
        }
        _ => println!("Unable to find nodes matching ids: {:?} & {:?}", from, to),
    }
}

pub fn disconnect(
    to: i64,
    to_input: Option<i64>,
    node_map: &HashMap<i64, Rc<RefCell<Node>>>,
) -> () {

    match node_map.get(&to) {
        Some(to_node) => to_node.borrow_mut().set_input(None, to_input),
        _ => println!("Unable to find nodes matching id: {:?}", to),
    }
}

pub fn pull(node: &mut Node) -> FlowData {
    node.pull()
}
