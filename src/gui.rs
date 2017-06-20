//! A simple example that demonstrates using conrod within a basic `winit` window loop, using
//! `glium` to render the `conrod::render::Primitives` to screen.
//!
//! Note that the event loop that we create in this example behaves the same as the example
//! `EventLoop` in the `examples/support` module. It has been inlined in order to provide an
//! example that does not depend on the `support` module.

pub mod feature {

    use gui_node;
    use build;
    use commands::{CreateNodeCommand, CreateConnectionCommand, Command, CommandGroup, UndoStack};
    use params::Params;
    use NodeUI;
    use NodeUIData;
    use widgets;

    use std::rc::Rc;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ops::DerefMut;

    use conrod;
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::{DisplayBuild, Surface};
    use conrod::graph::Walker;
    use std;

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
            parameters_title,
            parameters_field,
        }
    }

    fn find_gui_node(id: conrod::widget::id::Id,
                     nodes: &HashMap<conrod::widget::id::Id, Rc<RefCell<gui_node::GuiNodeData>>>)
                     -> Option<&Rc<RefCell<gui_node::GuiNodeData>>> {
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
        const FONT_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"),
                                                "/assets/fonts/NotoSans/NotoSans-Regular.ttf");
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
            display_menu: false,
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
            connections: vec![],
            selected_node: None,
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

                    match event {
                        Input::Release(Button::Keyboard(Key::A)) => {
                            if !params.display_menu {
                                params.display_menu = true;
                                if let Some(index) = params.selected_node {
                                    if let Some(node) = find_gui_node(index, &params.gui_nodes) {
                                        let b = node.borrow();
                                        params.tab_x = b.x + 200.0;
                                        params.tab_y = b.y;
                                    }
                                } else {
                                    params.tab_x = params.mouse_x;
                                    params.tab_y = params.mouse_y;
                                }
                            }
                        }
                        Input::Release(Button::Keyboard(Key::U)) => {
                            if !params.display_menu {
                                undo_stack.undo(&mut params);
                            }
                        }
                        Input::Release(Button::Keyboard(Key::R)) => {
                            if !params.display_menu {
                                let global = ui.global_input();
                                let ref state = global.current;
                                if state.modifiers.contains(conrod::input::keyboard::CTRL) {
                                    undo_stack.redo(&mut params);
                                }
                            }
                        }
                        Input::Release(Button::Keyboard(Key::Backspace)) |
                        Input::Release(Button::Keyboard(Key::Delete)) => {}
                        Input::Motion(Motion::MouseCursor { x, y }) => {
                            params.mouse_x = x as f64;
                            params.mouse_y = y as f64;
                        }
                        Input::Release(Button::Keyboard(Key::Escape)) => {
                            params.display_menu = false;
                            params.name_input = String::from("");
                        }
                        _ => {}
                    }
                }

                match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::Event::KeyboardInput(_,
                                                        _,
                                                        Some(glium::glutin::VirtualKeyCode::Q)) |
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

    fn set_ui(ref mut ui: conrod::UiCell,
              ids: &mut Ids,
              params: &mut Params,
              undo_stack: &mut UndoStack) {
        use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};
        use conrod::position::{Position, Relative};

        widget::Canvas::new()
            .color(color::DARK_CHARCOAL)
            .flow_right(&[(ids.node_panel, widget::Canvas::new().length(500.0).color(color::RED)),
                          (ids.parameters_panel,
                           widget::Canvas::new().length(300.0).color(color::BLUE))])
            .set(ids.canvas, ui);

        for event in widgets::Background::new()
                .parent(ids.node_panel)
                .set(ids.node_background, ui) {
            match event {
                widgets::Event::Click => {
                    params.selected_node = None;
                }
                _ => {}
            }
        }

        if let Some(id) = params.selected_node {
            if let Some(g_node) = params.gui_nodes.get(&id) {
                let n = g_node.borrow();
                if let Some(node) = params.node_map.get(&n.node_id) {
                    let mut nn = node.borrow_mut();
                    let param_ui = nn.get_ui();
                    match param_ui {
                        NodeUI::None => {
                            widget::Text::new("Nothing")
                                .parent(ids.parameters_panel)
                                .middle_of(ids.parameters_panel)
                                .set(ids.parameters_title, ui);
                        }
                        NodeUI::StringField(data) => {
                            if let NodeUIData::StringData(value) = nn.get_value(&data.field) {
                                for event in widget::TextBox::new(value.as_str())
                                        .parent(ids.parameters_panel)
                                        .middle_of(ids.parameters_panel)
                                        .color(color::WHITE)
                                        .w(200.0)
                                        .h(30.0)
                                        .left_justify()
                                        .set(ids.parameters_field, ui) {

                                    match event {
                                        widget::text_box::Event::Update(string) => {
                                            nn.set_value(&data.field,
                                                         NodeUIData::StringData(string));
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if params.display_menu {
            widget::Canvas::new()
                .color(color::RED)
                .pad(5.0)
                .w(200.0)
                .h(30.0)
                .x_position(Position::Relative(Relative::Scalar(params.tab_x), Some(ids.canvas)))
                .y_position(Position::Relative(Relative::Scalar(params.tab_y), Some(ids.canvas)))
                .set(ids.name_input_background, ui);

            for event in widget::TextBox::new(params.name_input.as_str())
                    .parent(ids.name_input_background)
                    .color(color::WHITE)
                    .top_left_of(ids.name_input_background)
                    .left_justify()
                    .set(ids.text_edit, ui) {

                match event {
                    widget::text_box::Event::Update(string) => params.name_input = string,
                    widget::text_box::Event::Enter => {
                        create_node(params, undo_stack, ui.widget_id_generator());
                        params.name_input = "".to_string();
                        params.display_menu = false;
                    }
                }
            }
        }

        fn in_box(mouse_xy: &conrod::position::Point, node_x: f64, node_y: f64) -> bool {
            return if mouse_xy[0] < (node_x - 400.0) || mouse_xy[0] > ((node_x - 400.0) + 140.0) {
                       false
                   } else if mouse_xy[1] > ((600.0 - node_y) - 300.0) ||
                             mouse_xy[1] < (((600.0 - node_y) - 300.0) - 30.0) {
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
            let selected = Some(id) == params.selected_node;
            for event in gui_node::GuiNode::new(g_node.clone(), selected)
                    .parent(ids.canvas)
                    .w(140.0)
                    .h(30.0)
                    .set(id, ui) {
                match event {
                    gui_node::Event::Click => {
                        params.selected_node = Some(id);
                    }
                    gui_node::Event::ConnectOutput => {
                        let global = ui.global_input();
                        let ref state = global.current;
                        params.connect_node = Some(g_node.clone());
                        params.current_connection = Some(state.mouse.xy);
                    }
                    gui_node::Event::ConnectInput => {

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

        fn find_node(id: i64,
                     nodes: &HashMap<conrod::widget::id::Id, Rc<RefCell<gui_node::GuiNodeData>>>)
                     -> Option<Rc<RefCell<gui_node::GuiNodeData>>> {
            for (_, node) in nodes.iter() {
                let n = node.borrow();
                if n.node_id == id {
                    return Some(node.clone());
                }
            }
            None
        }

        fn calculate_point_path(start: conrod::position::Point,
                                end: conrod::position::Point)
                                -> Vec<conrod::position::Point> {
            if end[0] >= start[0] + 40.0 {
                let x_halfway = (start[0] + end[0]) / 2.0;
                vec![start, [x_halfway, start[1]], [x_halfway, end[1]], end]
            } else {
                let y_halfway = (start[1] + end[1]) / 2.0;
                vec![start,
                     [start[0] + 20.0, start[1]],
                     [start[0] + 20.0, y_halfway],
                     [end[0] - 20.0, y_halfway],
                     [end[0] - 20.0, end[1]],
                     end]
            }
        }

        for connection in &params.connections {
            match (find_node(connection.from, &params.gui_nodes),
                   find_node(connection.to, &params.gui_nodes)) {
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

    fn create_node(mut params: &mut Params,
                   mut undo_stack: &mut UndoStack,
                   mut generator: conrod::widget::id::Generator)
                   -> () {
        let node_id = params.node_id + 1;
        params.node_id = params.node_id + 1;
        let maybe_node = build::build(node_id, params.name_input.clone());
        if let Some(node) = maybe_node {
            let id = generator.next();
            let g_node = Rc::new(RefCell::new(gui_node::GuiNodeData {
                                                  id: id,
                                                  node_id: node_id,
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

            if let Some(index) = params.selected_node {
                if let Some(node) = find_gui_node(index, &params.gui_nodes) {
                    let b = node.borrow();
                    let connection_id;
                    {
                        connection_id = generator.next();
                    }

                    let command =
                        CreateConnectionCommand::new_ref(connection_id, b.node_id, node_id);

                    commands.push(command);


                }
            }

            let command_group = CommandGroup::new_ref(commands);

            let mut com = command_group.borrow_mut();
            com.execute(&mut params);
            undo_stack.push(command_group.clone());

            params.selected_node = Some(g_node_deref.id);
        }
    }

}
