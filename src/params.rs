
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use conrod;

use gui_node;
use Node;
use gui::feature::Connection;


pub struct Params {
    pub node_id: i64,
    pub display_menu: bool,
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
    pub connections: Vec<Connection>,
    pub selected_node: Option<conrod::widget::id::Id>,
}
