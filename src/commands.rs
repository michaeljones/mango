
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
    fn undo(&mut self, params: &mut Params);
}

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
        let node = self.node.borrow();
        let g_node = self.g_node.borrow();
        params.node_map.insert(node.id(), self.node.clone());
        params.gui_nodes.insert(g_node.id, self.g_node.clone());

        self.previous_last_node = params.last_node.clone();
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
        build::connect(self.from, None, self.to, Some(1), &params.node_map);
        params
            .connections
            .push(Connection {
                      id: self.id,
                      from: self.from,
                      to: self.to,
                  });
    }

    fn undo(&mut self, params: &mut Params) {
        build::disconnect(self.to, Some(1), &params.node_map);
        params.connections.pop();
    }
}

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
            com.execute(&mut params);
            self.undo.push(command.clone());
        }
    }
}
