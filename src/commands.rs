
use std::rc::Rc;
use std::cell::RefCell;

use conrod;

use gui::feature::Connection;
use params::Params;
use build;
use NodeRef;
use gui_node;


pub trait Command {
    fn execute(&mut self, &mut Params);
    fn redo(&mut self, params: &mut Params);
    fn undo(&mut self, params: &mut Params);

    fn is_undoable(&self) -> bool { true }
}


// CreateNodeCommand
//
pub struct CreateNodeCommand {
    node: NodeRef,
    g_node: gui_node::GuiNodeDataRef,
    previous_last_node: Option<Rc<RefCell<gui_node::GuiNodeData>>>,
}

impl CreateNodeCommand {
    pub fn new(node: NodeRef, g_node: gui_node::GuiNodeDataRef) -> Self {
        CreateNodeCommand {
            node: node,
            g_node: g_node,
            previous_last_node: None,
        }
    }

    pub fn new_ref(node: NodeRef, g_node: gui_node::GuiNodeDataRef) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(CreateNodeCommand::new(node, g_node)))
    }
}

impl Command for CreateNodeCommand {
    fn execute(&mut self, params: &mut Params) {
        self.previous_last_node = params.last_node.clone();
        self.redo(params);
    }

    fn redo(&mut self, params: &mut Params) {
        let node = self.node.borrow();
        let g_node = self.g_node.borrow();
        params.node_map.insert(node.id(), self.node.clone());
        params.gui_nodes.insert(g_node.id, self.g_node.clone());
        params.last_node = Some(self.g_node.clone());
    }

    fn undo(&mut self, params: &mut Params) {
        let node = self.node.borrow();
        let g_node = self.g_node.borrow();
        params.node_map.remove(&node.id());
        params.gui_nodes.remove(&g_node.id);

        params.last_node = self.previous_last_node.clone();
    }
}

// DeleteNodeCommand
//
pub struct DeleteNodeCommand {
    node: NodeRef,
    g_node: gui_node::GuiNodeDataRef,
    previous_last_node: Option<Rc<RefCell<gui_node::GuiNodeData>>>,
}

impl DeleteNodeCommand {
    pub fn new(node: NodeRef, g_node: gui_node::GuiNodeDataRef) -> Self {
        DeleteNodeCommand {
            node: node,
            g_node: g_node,
            previous_last_node: None,
        }
    }

    pub fn new_ref(node: NodeRef, g_node: gui_node::GuiNodeDataRef) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(DeleteNodeCommand::new(node, g_node)))
    }
}

impl Command for DeleteNodeCommand {
    fn execute(&mut self, params: &mut Params) {
        self.redo(params);
    }

    fn redo(&mut self, params: &mut Params) {
        let node = self.node.borrow();
        let g_node = self.g_node.borrow();
        params.node_map.remove(&node.id());
        params.gui_nodes.remove(&g_node.id);
    }

    fn undo(&mut self, params: &mut Params) {
        let node = self.node.borrow();
        let g_node = self.g_node.borrow();
        params.node_map.insert(node.id(), self.node.clone());
        params.gui_nodes.insert(g_node.id, self.g_node.clone());
    }
}

// CreateConnectionCommand
//
pub struct CreateConnectionCommand {
    id: conrod::widget::id::Id,
    from: i64,
    to: i64,
}

impl CreateConnectionCommand {
    pub fn new(id: conrod::widget::id::Id, from: i64, to: i64) -> Self {
        CreateConnectionCommand {
            id: id,
            from: from,
            to: to,
        }
    }

    pub fn new_ref(id: conrod::widget::id::Id, from: i64, to: i64) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(CreateConnectionCommand::new(id, from, to)))
    }
}

impl Command for CreateConnectionCommand {
    fn execute(&mut self, mut params: &mut Params) {
        self.redo(params)
    }

    fn redo(&mut self, mut params: &mut Params) {
        build::connect(self.from, None, self.to, Some(1), &params.node_map);

        params
            .connections
            .insert((self.from, self.to),
                    Connection {
                        id: self.id,
                        from: self.from,
                        to: self.to,
                    });
    }

    fn undo(&mut self, params: &mut Params) {
        build::disconnect(self.to, Some(1), &params.node_map);
        params.connections.remove(&(self.from, self.to));
    }
}

// DisconnectCommand
//
pub struct DisconnectCommand {
    from: i64,
    to: i64,
    connection: Option<Connection>,
}

impl DisconnectCommand {
    pub fn new(from: i64, to: i64) -> Self {
        DisconnectCommand {
            from: from,
            to: to,
            connection: None,
        }
    }

    pub fn new_ref(from: i64, to: i64) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(DisconnectCommand::new(from, to)))
    }
}

impl Command for DisconnectCommand {
    fn execute(&mut self, mut params: &mut Params) {
        self.connection = params.connections.remove(&(self.from, self.to));
        self.redo(params)
    }

    fn redo(&mut self, mut params: &mut Params) {
        build::disconnect(self.to, Some(1), &params.node_map);
    }

    fn undo(&mut self, params: &mut Params) {
        build::connect(self.from, None, self.to, Some(1), &params.node_map);

        if let Some(ref conn) = self.connection {
            // I can't figure out how to clone the old connection so I have to create a new one
            // with the same data to insert into the map
            params
                .connections
                .insert((self.from, self.to),
                        Connection {
                            id: conn.id,
                            from: self.from,
                            to: self.to,
                        });
        }
    }
}

// Command Group
//
pub struct CommandGroup {
    commands: Vec<Rc<RefCell<Command>>>,
}

impl CommandGroup {
    pub fn new(commands: Vec<Rc<RefCell<Command>>>) -> Self {
        CommandGroup { commands: commands }
    }

    pub fn new_ref(commands: Vec<Rc<RefCell<Command>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(CommandGroup::new(commands)))
    }
}

impl Command for CommandGroup {
    fn execute(&mut self, mut params: &mut Params) {
        for command in &self.commands {
            let mut com = command.borrow_mut();
            com.execute(&mut params);
        }
    }

    fn redo(&mut self, mut params: &mut Params) {
        for command in &self.commands {
            let mut com = command.borrow_mut();
            com.redo(&mut params);
        }
    }

    fn undo(&mut self, mut params: &mut Params) {
        for command in self.commands.iter().rev() {
            let mut com = command.borrow_mut();
            com.undo(&mut params);
        }
    }
}

pub struct UndoStack {
    undo: Vec<Rc<RefCell<Command>>>,
    redo: Vec<Rc<RefCell<Command>>>,
}

impl UndoStack {
    pub fn new() -> Self {
        UndoStack {
            undo: vec![],
            redo: vec![],
        }
    }

    pub fn push(&mut self, command: Rc<RefCell<Command>>) {
        self.undo.push(command.clone());
        self.redo.clear();
    }

    pub fn undo(&mut self, mut params: &mut Params) {
        if let Some(command) = self.undo.pop() {
            let mut com = command.borrow_mut();
            com.undo(&mut params);
            self.redo.push(command.clone());
        }
    }

    pub fn redo(&mut self, mut params: &mut Params) {
        if let Some(command) = self.redo.pop() {
            let mut com = command.borrow_mut();
            com.redo(&mut params);
            self.undo.push(command.clone());
        }
    }
}
