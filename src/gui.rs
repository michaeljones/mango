//! A simple example that demonstrates using conrod within a basic `winit` window loop, using
//! `glium` to render the `conrod::render::Primitives` to screen.
//!
//! Note that the event loop that we create in this example behaves the same as the example
//! `EventLoop` in the `examples/support` module. It has been inlined in order to provide an
//! example that does not depend on the `support` module.

pub mod feature {

    use gui_node;
    use build;
    use commands::{CreateNodeCommand, CreateConnectionCommand, DisconnectCommand,
                   DeleteNodeCommand, Command, CommandGroup, UndoStack};
    use params::{Params, CreateState, CommandLine, InteractionMode};
    use Node;
    use NodeUI;
    use NodeUIData;
    use widgets;
    use commandline;

    use std::rc::Rc;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ops::DerefMut;
    use std::ops::Deref;

    use conrod;
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::{DisplayBuild, Surface};
    use conrod::graph::Walker;
    use std;

    #[derive(Debug)]
    pub struct Connection {
        pub id: conrod::widget::id::Id,
        pub from: i64,
        pub to: i64,
    }

    widget_ids! {
        struct Ids {
            canvas,
            text_edit,
            scrollbar,
            name_input_background,
            nodes[],
            line,
            node_panel,
            node_background,
            parameters_panel,
            command_line,
        }
    }

    fn find_gui_node(
        id: conrod::widget::id::Id,
        nodes: &HashMap<conrod::widget::id::Id, Rc<RefCell<gui_node::GuiNodeData>>>,
    ) -> Option<&Rc<RefCell<gui_node::GuiNodeData>>> {
        nodes.get(&id)
    }


    pub fn gui() {
        const WIDTH: u32 = 800;
        const HEIGHT: u32 = 600;

        // Build the window.
        let display = glium::glutin::WindowBuilder::new()
            .with_vsync()
            .with_dimensions(WIDTH, HEIGHT)
            .with_title("slipstream")
            .with_multisampling(4)
            .build_glium()
            .unwrap();

        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        // Generate the widget identifiers.
        let mut ids = Ids::new(ui.widget_id_generator());

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        const FONT_PATH: &'static str = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/NotoSans/NotoSans-Regular.ttf"
        );
        ui.fonts.insert_from_file(FONT_PATH).unwrap();

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // The image map describing each of our widget->image mappings (in our case, none).
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        // Poll events from the window.
        let mut last_update = std::time::Instant::now();
        let mut ui_needs_update = true;

        let mut params = Params {
            node_id: 0,
            display_menu: CreateState::None,
            mouse_x: 0.0,
            mouse_y: 0.0,
            tab_x: 0.0,
            tab_y: 0.0,
            name_input: String::new(),
            gui_nodes: HashMap::new(),
            last_node: None,
            connect_node: None,
            node_map: HashMap::new(),
            current_connection: None,
            connections: HashMap::new(),
            selected_nodes: vec![],
            command_line: CommandLine::None,
            interaction_mode: InteractionMode::Normal,
        };

        let mut undo_stack = UndoStack::new();

        'main: loop {

            // We don't want to loop any faster than 60 FPS, so wait until it has been at least
            // 16ms since the last yield.
            let sixteen_ms = std::time::Duration::from_millis(16);
            let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            // Collect all pending events.
            let mut events: Vec<_> = display.poll_events().collect();

            // If there are no events and the `Ui` does not need updating, wait for the next event.
            if events.is_empty() && !ui_needs_update {
                events.extend(display.wait_events().next());
            }

            // Reset the needs_update flag and time this update.
            ui_needs_update = false;
            last_update = std::time::Instant::now();

            // Handle all events.
            for event in events {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                    use conrod::event::Input;
                    use conrod::input::{Button, Key, Motion};

                    ui.handle_event(event.clone());
                    ui_needs_update = true;

                    let entering_text = params.display_menu != CreateState::None ||
                        params.command_line != CommandLine::None;

                    if entering_text {
                        match event.clone() {
                            Input::Release(Button::Keyboard(Key::Escape)) => {
                                if params.display_menu != CreateState::None {
                                    params.display_menu = CreateState::None;
                                    params.name_input = String::from("");
                                } else if params.command_line != CommandLine::None {
                                    params.command_line = CommandLine::None;
                                }
                            }
                            _ => {}
                        }
                    } else {
                        match event.clone() {
                            Input::Release(Button::Keyboard(Key::V)) => {
                                params.interaction_mode = InteractionMode::Visual;
                            }
                            Input::Release(Button::Keyboard(Key::Escape)) => {
                                params.interaction_mode = InteractionMode::Normal;
                            }
                            Input::Release(Button::Keyboard(Key::A)) => {
                                match params.selected_nodes.len() {
                                    0 => {
                                        params.display_menu = CreateState::Free;
                                        params.tab_x = params.mouse_x;
                                        params.tab_y = params.mouse_y;
                                    }
                                    1 => {
                                        let index = params.selected_nodes[0];
                                        params.display_menu = CreateState::After;
                                        if let Some(node) = find_gui_node(
                                            index,
                                            &params.gui_nodes,
                                        ) {
                                            let b = node.borrow();
                                            params.tab_x = b.x + 200.0;
                                            params.tab_y = b.y;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Input::Release(Button::Keyboard(Key::I)) => {
                                match params.selected_nodes.len() {
                                    0 => {
                                        params.display_menu = CreateState::Free;
                                        params.tab_x = params.mouse_x;
                                        params.tab_y = params.mouse_y;
                                    }
                                    1 => {
                                        let index = params.selected_nodes[0];
                                        params.display_menu = CreateState::Before;
                                        if let Some(node) = find_gui_node(
                                            index,
                                            &params.gui_nodes,
                                        ) {
                                            let b = node.borrow();
                                            params.tab_x = b.x - 200.0;
                                            params.tab_y = b.y;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Input::Release(Button::Keyboard(Key::S)) => {
                                match params.selected_nodes.len() {
                                    0 => {
                                        params.display_menu = CreateState::Free;
                                        params.tab_x = params.mouse_x;
                                        params.tab_y = params.mouse_y;
                                    }
                                    1 => {
                                        let index = params.selected_nodes[0];
                                        params.display_menu = CreateState::Substitute;
                                        if let Some(node) = find_gui_node(
                                            index,
                                            &params.gui_nodes,
                                        ) {
                                            let b = node.borrow();
                                            params.tab_x = b.x;
                                            params.tab_y = b.y;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Input::Release(Button::Keyboard(Key::U)) => {
                                undo_stack.undo(&mut params);
                            }
                            Input::Release(Button::Keyboard(Key::R)) => {
                                let global = ui.global_input();
                                let ref state = global.current;
                                if state.modifiers.contains(conrod::input::keyboard::CTRL) {
                                    undo_stack.redo(&mut params);
                                }
                            }
                            Input::Release(Button::Keyboard(Key::H)) => {
                                match params.selected_nodes.len() {
                                    1 => {
                                        let selected_node = params.selected_nodes[0];
                                        if let Some(g_node) = params.gui_nodes.get(&selected_node) {
                                            let gn = g_node.borrow();
                                            if let Some(input_node_id) =
                                                find_input_node(gn.node_id, &params.connections)
                                            {
                                                for (_key, g_node) in &params.gui_nodes {
                                                    let gnn = g_node.borrow();
                                                    if gnn.node_id == input_node_id {
                                                        params.selected_nodes = vec![gnn.id];
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Input::Release(Button::Keyboard(Key::L)) => {
                                match params.selected_nodes.len() {
                                    1 => {
                                        let selected_node = params.selected_nodes[0];
                                        if let Some(g_node) = params.gui_nodes.get(&selected_node) {
                                            let gn = g_node.borrow();
                                            if let Some(input_node_id) =
                                                find_output_node(gn.node_id, &params.connections)
                                            {
                                                for (_key, g_node) in &params.gui_nodes {
                                                    let gnn = g_node.borrow();
                                                    if gnn.node_id == input_node_id {
                                                        params.selected_nodes = vec![gnn.id];
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Input::Text(text) => {
                                if text == ":" {
                                    if params.display_menu == CreateState::None {
                                        params.command_line = CommandLine::Text(String::from(""))
                                    }
                                }
                            }
                            _ => {
                                // println!("{:?}", event);
                            }
                        }

                    }
                    match event.clone() {
                        Input::Motion(Motion::MouseCursor { x, y }) => {
                            params.mouse_x = x as f64;
                            params.mouse_y = y as f64;
                        }
                        _ => {}
                    }
                }

                match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::Event::KeyboardInput(
                        _,
                        _,
                        Some(glium::glutin::VirtualKeyCode::Q),
                    ) |
                    glium::glutin::Event::Closed => {
                        if let Some(g_node) = params.last_node {
                            if let Some(node) = params.node_map.get(&g_node.borrow().node_id) {
                                build::pull(node.borrow_mut().deref_mut());
                            } else {
                                println!("failed to find node in node_map");
                            }

                        } else {
                            println!("failed to find last node");
                        }
                        break 'main;
                    }
                    _ => {}
                }
            }

            // Instantiate all widgets in the GUI.
            {
                set_ui(ui.set_widgets(), &mut ids, &mut params, &mut undo_stack);
            }

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(10.0 / 256.0, 10.0 / 256.0, 10.0 / 256.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }

    fn draw_parameters(
        _gn: &Rc<RefCell<gui_node::GuiNodeData>>,
        node: &Rc<RefCell<Node>>,
        parent_id: conrod::widget::id::Id,
        param_ui: &NodeUI,
        ids: &mut conrod::widget::id::List,
        ui: &mut conrod::UiCell,
    ) {
        use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};

        match param_ui {
            &NodeUI::None => {
                let id = ids.walk().next(ids, &mut ui.widget_id_generator());
                widget::Text::new("Nothing")
                    .parent(parent_id)
                    .middle_of(parent_id)
                    .set(id, ui);
            }
            &NodeUI::StringField(ref data) => {
                let mut bn = node.borrow_mut();
                let mut nn = bn.deref_mut();
                let id = ids.walk().next(ids, &mut ui.widget_id_generator());

                if let NodeUIData::StringData(value) = nn.get_value(&data.field) {
                    for event in widget::TextBox::new(value.as_str())
                        .parent(parent_id)
                        .middle_of(parent_id)
                        .color(color::WHITE)
                        .w(200.0)
                        .h(30.0)
                        .left_justify()
                        .set(id, ui)
                    {

                        match event {
                            widget::text_box::Event::Update(string) => {
                                nn.set_value(&data.field, NodeUIData::StringData(string));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

    }

    fn set_ui(
        ref mut ui: conrod::UiCell,
        ids: &mut Ids,
        params: &mut Params,
        undo_stack: &mut UndoStack,
    ) {
        use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};
        use conrod::position::{Position, Relative};

        widget::Canvas::new()
            .color(color::DARK_CHARCOAL)
            .flow_right(
                &[
                    (
                        ids.node_panel,
                        widget::Canvas::new().length(500.0).color(color::RED),
                    ),
                    (
                        ids.parameters_panel,
                        widget::Canvas::new()
                            .length(300.0)
                            .rgb(91.0 / 256.0, 103.0 / 256.0, 107.0 / 256.0),
                    ),
                ],
            )
            .set(ids.canvas, ui);

        for event in widgets::Background::new()
            .parent(ids.node_panel)
            .set(ids.node_background, ui)
        {
            match event {
                widgets::Event::Click => {
                    params.selected_nodes = vec![];
                }
                _ => {}
            }
        }

        match params.selected_nodes.len() {
            1 => {
                let id = params.selected_nodes[0];
                if let Some(g_node) = params.gui_nodes.get(&id) {
                    let mut gn = g_node.borrow_mut();
                    let node_id = gn.node_id;
                    if let Some(node) = params.node_map.get(&node_id) {
                        let param_ui;
                        {
                            let bn = node.borrow();
                            let nn = bn.deref();
                            param_ui = nn.get_ui();
                        }
                        draw_parameters(
                            &g_node,
                            &node,
                            ids.parameters_panel,
                            &param_ui,
                            &mut gn.parameter_ids,
                            ui,
                        );
                    }
                }
            }
            _ => {}
        }

        if params.display_menu != CreateState::None {
            widget::Canvas::new()
                .color(color::RED)
                .pad(5.0)
                .w(200.0)
                .h(30.0)
                .x_position(Position::Relative(
                    Relative::Scalar(params.tab_x),
                    Some(ids.canvas),
                ))
                .y_position(Position::Relative(
                    Relative::Scalar(params.tab_y),
                    Some(ids.canvas),
                ))
                .set(ids.name_input_background, ui);

            for event in widget::TextBox::new(params.name_input.as_str())
                .parent(ids.name_input_background)
                .color(color::WHITE)
                .top_left_of(ids.name_input_background)
                .left_justify()
                .set(ids.text_edit, ui)
            {

                match event {
                    widget::text_box::Event::Update(string) => params.name_input = string,
                    widget::text_box::Event::Enter => {
                        create_node(params, undo_stack, ui.widget_id_generator());
                        params.name_input = "".to_string();
                        params.display_menu = CreateState::None;
                    }
                }
            }
        }

        if let CommandLine::Text(text) = params.command_line.clone() {
            for event in widget::TextBox::new(text.as_str())
                .parent(ids.node_panel)
                .color(color::WHITE)
                .h(30.0)
                .bottom_left_of(ids.node_panel)
                .left_justify()
                .set(ids.command_line, ui)
            {

                match event {
                    widget::text_box::Event::Update(string) => {
                        params.command_line = CommandLine::Text(string);
                    }
                    widget::text_box::Event::Enter => {
                        if commandline::run(&text, params, undo_stack) {
                            params.command_line = CommandLine::None;
                        }
                    }
                }
            }
        }

        fn in_box(mouse_xy: &conrod::position::Point, node_x: f64, node_y: f64) -> bool {
            return if mouse_xy[0] < (node_x - 400.0) || mouse_xy[0] > ((node_x - 400.0) + 140.0) {
                false
            } else if mouse_xy[1] > ((600.0 - node_y) - 300.0) ||
                       mouse_xy[1] < (((600.0 - node_y) - 300.0) - 30.0)
            {
                false
            } else {
                true
            };
        }

        for (_, g_node) in params.gui_nodes.iter() {
            let id;
            let node_id;
            {
                let node = g_node.borrow();
                id = node.id;
                node_id = node.node_id;
            }
            let selected = vec![id] == params.selected_nodes;
            for event in gui_node::GuiNode::new(g_node.clone(), selected)
                .parent(ids.canvas)
                .w(140.0)
                .h(30.0)
                .set(id, ui)
            {
                match event {
                    gui_node::Event::Click => {
                        params.selected_nodes = vec![id];
                    }
                    gui_node::Event::ConnectOutput => {
                        let global = ui.global_input();
                        let ref state = global.current;
                        params.connect_node = Some(g_node.clone());
                        params.current_connection = Some(state.mouse.xy);
                    }
                    gui_node::Event::ConnectInput => {

                        /*
                        let mut nnn: Option<&Rc<RefCell<gui_node::GuiNodeData>>> = None;

                        {
                            let graph = ui.widget_graph();
                            let mut walker = graph.children(ids.canvas);
                            let global = ui.global_input();
                            let ref state = global.current;
                            loop {
                                // Walk the graph to find all nodes
                                if let Some(node_index) = walker.next_node(graph) {
                                    // If the node index corresponds to a gui_node
                                    match (find_gui_node(node_index, &params.gui_nodes),
                                           graph.node(node_index)) {
                                        (Some(ref gui_node),
                                         Some(&conrod::graph::Node::Widget(ref _container))) => {
                                            let m = gui_node.borrow();
                                            if in_box(&state.mouse.xy, m.x, m.y) {
                                                nnn = Some(gui_node.clone());
                                                break;
                                            }
                                        }
                                        _ => {
                                            println!("No match");
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }

                        }
                        match nnn {
                            Some(ref node) => {
                                let nn = node.borrow();
                                build::connect(node_id,
                                               None,
                                               nn.node_id,
                                               Some(1),
                                               &params.node_map);
                                let connection_id;
                                {
                                    let mut generator = ui.widget_id_generator();
                                    connection_id = generator.next();
                                }
                                params
                                    .connections
                                    .push(Connection {
                                              id: connection_id,
                                              from: node_id,
                                              to: nn.node_id,
                                          });
                            }
                            None => {}
                        }
                        params.current_connection = None;
                        */
                    }
                    _ => {}
                }
            }

        }

        match params.current_connection {
            Some(xy) => {
                let point;
                {
                    let global = ui.global_input();
                    let ref state = global.current;
                    point = [state.mouse.xy[0], state.mouse.xy[1]]
                }
                widget::primitive::line::Line::new(xy, point)
                    .top_left_of(ids.canvas)
                    .set(ids.line, ui);
            }
            None => {}
        }

        fn find_node(
            id: i64,
            nodes: &HashMap<conrod::widget::id::Id, Rc<RefCell<gui_node::GuiNodeData>>>,
        ) -> Option<Rc<RefCell<gui_node::GuiNodeData>>> {
            for (_, node) in nodes.iter() {
                let n = node.borrow();
                if n.node_id == id {
                    return Some(node.clone());
                }
            }
            None
        }

        fn calculate_point_path(
            start: conrod::position::Point,
            end: conrod::position::Point,
        ) -> Vec<conrod::position::Point> {
            if end[0] >= start[0] + 40.0 {
                let x_halfway = (start[0] + end[0]) / 2.0;
                vec![start, [x_halfway, start[1]], [x_halfway, end[1]], end]
            } else {
                let y_halfway = (start[1] + end[1]) / 2.0;
                vec![
                    start,
                    [start[0] + 20.0, start[1]],
                    [start[0] + 20.0, y_halfway],
                    [end[0] - 20.0, y_halfway],
                    [end[0] - 20.0, end[1]],
                    end,
                ]
            }
        }

        for (_key, connection) in &params.connections {
            match (
                find_node(connection.from, &params.gui_nodes),
                find_node(connection.to, &params.gui_nodes),
            ) {
                (Some(a), Some(b)) => {
                    let an = a.borrow();
                    let bn = b.borrow();
                    let start = [an.x + 70.0 - 10.0, an.y];
                    let end = [bn.x - 70.0 + 10.0, bn.y];
                    let points = calculate_point_path(start, end);
                    widget::primitive::point_path::PointPath::new(points)
                        .top_left_of(ids.canvas)
                        .thickness(2.0)
                        .set(connection.id, ui);
                }
                _ => {
                    println!("Failed to find nodes");
                }
            }
        }


        widget::Scrollbar::y_axis(ids.canvas)
            .auto_hide(true)
            .set(ids.scrollbar, ui);
    }

    fn find_input_node(id: i64, connections: &HashMap<(i64, i64), Connection>) -> Option<i64> {
        for (_key, connection) in connections {
            if connection.to == id {
                return Some(connection.from);
            }
        }
        None
    }

    fn find_output_node(id: i64, connections: &HashMap<(i64, i64), Connection>) -> Option<i64> {
        for (_key, connection) in connections {
            if connection.from == id {
                return Some(connection.to);
            }
        }
        None
    }

    fn create_node(
        mut params: &mut Params,
        mut undo_stack: &mut UndoStack,
        mut generator: conrod::widget::id::Generator,
    ) -> () {
        let new_node_id = params.node_id + 1;
        params.node_id = params.node_id + 1;
        let maybe_node = build::build(new_node_id, params.name_input.clone());
        if let Some(node) = maybe_node {
            let id = generator.next();
            let g_node = Rc::new(RefCell::new(gui_node::GuiNodeData {
                id: id,
                parameter_ids: conrod::widget::id::List::new(),
                node_id: new_node_id,
                label: params.name_input.clone(),
                x: params.tab_x,
                y: params.tab_y,
                origin_x: params.tab_x,
                origin_y: params.tab_y,
                mode: gui_node::Mode::None,
            }));

            let g_node_deref = g_node.borrow();


            let mut commands: Vec<Rc<RefCell<Command>>> = vec![];
            let command = CreateNodeCommand::new_ref(node.clone(), g_node.clone());

            commands.push(command);

            match params.selected_nodes.len() {
                1 => {
                    let index = params.selected_nodes[0];
                    if let Some(g_node) = find_gui_node(index, &params.gui_nodes) {
                        let b = g_node.borrow();
                        let connection_id = generator.next();

                        match params.display_menu {
                            CreateState::Before => {
                                if let Some(connected) = find_input_node(
                                    b.node_id,
                                    &params.connections,
                                ) {
                                    commands.push(CreateConnectionCommand::new_ref(
                                        generator.next(),
                                        connected,
                                        new_node_id,
                                    ));

                                    commands.push(DisconnectCommand::new_ref(connected, b.node_id));
                                }

                                commands.push(CreateConnectionCommand::new_ref(
                                    connection_id,
                                    new_node_id,
                                    b.node_id,
                                ));
                            }
                            CreateState::After => {
                                let command = CreateConnectionCommand::new_ref(
                                    connection_id,
                                    b.node_id,
                                    new_node_id,
                                );
                                commands.push(command);
                            }
                            CreateState::Substitute => {
                                let input_node = find_input_node(b.node_id, &params.connections);
                                let output_node = find_output_node(b.node_id, &params.connections);
                                let node = params.node_map.get(&b.node_id);
                                match (node, input_node, output_node) {
                                    (Some(nn), Some(inode), Some(onode)) => {
                                        commands.push(DisconnectCommand::new_ref(inode, b.node_id));
                                        commands.push(DisconnectCommand::new_ref(b.node_id, onode));
                                        commands.push(CreateConnectionCommand::new_ref(
                                            generator.next(),
                                            inode,
                                            new_node_id,
                                        ));
                                        commands.push(CreateConnectionCommand::new_ref(
                                            generator.next(),
                                            new_node_id,
                                            onode,
                                        ));
                                        commands.push(
                                            DeleteNodeCommand::new_ref(nn.clone(), g_node.clone()),
                                        );
                                    }
                                    _ => {}
                                }
                            }
                            CreateState::None => {}
                            CreateState::Free => {}
                        }
                    }
                }
                _ => {}
            }

            let command_group = CommandGroup::new_ref(commands);

            let mut com = command_group.borrow_mut();
            com.execute(&mut params);
            undo_stack.push(command_group.clone());

            params.selected_nodes = vec![g_node_deref.id];
        }
    }

}
