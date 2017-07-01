
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use conrod;

use gui_node;
use Node;
use gui::Connection;

#[derive(PartialEq)]
pub enum CreateState {
    None,
    Before,
    After,
    Substitute,
    Free,
}


#[derive(Debug, PartialEq, Clone)]
pub enum CommandLine {
    None,
    Text(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum InteractionMode {
    Normal,
    Visual,
}

pub struct Params {
    pub node_id: i64,
    pub display_menu: CreateState,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub tab_x: f64,
    pub tab_y: f64,
    pub name_input: String,
    pub gui_nodes: HashMap<conrod::widget::id::Id, Rc<RefCell<gui_node::GuiNodeData>>>,
    pub last_node: Option<Rc<RefCell<gui_node::GuiNodeData>>>,
    pub connect_node: Option<Rc<RefCell<gui_node::GuiNodeData>>>,
    pub node_map: HashMap<i64, Rc<RefCell<Node>>>,
    pub current_connection: Option<conrod::position::Point>,
    pub connections: HashMap<(i64, i64), Connection>,
    pub selected_nodes: Vec<conrod::widget::id::Id>,
    pub command_line: CommandLine,
    pub interaction_mode: InteractionMode,
}
