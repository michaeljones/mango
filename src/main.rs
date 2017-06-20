
extern crate json;
extern crate clap;

#[macro_use]
extern crate conrod;

use std::rc::Rc;
use std::cell::RefCell;

mod nodes;
mod gui;
mod gui_node;
mod build;
mod commands;
mod params;
mod widgets;

#[derive(Debug, Clone)]
pub enum FlowData {
    None,
    Error(String),
    String(String),
    StringArray(Vec<String>),
    Int(i64),
    IntArray(Vec<i64>),
    Json(json::JsonValue),
}

pub struct StringFieldData {
    pub label: String,
    pub field: String,
}

pub enum NodeUI {
    None,
    StringField(StringFieldData),
}

pub trait Node {
    fn id(&self) -> i64;
    fn pull(&mut self) -> FlowData;
    fn set_input(&mut self, node: Option<Rc<RefCell<Node>>>, index: Option<i64>) -> ();
    fn get_ui(&self) -> NodeUI {
        NodeUI::None
    }
}

type NodeRef = Rc<RefCell<Node>>;

fn main() {
    gui::feature::gui();
}
