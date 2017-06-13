//! A simple example that demonstrates using conrod within a basic `winit` window loop, using
//! `glium` to render the `conrod::render::Primitives` to screen.
//!
//! Note that the event loop that we create in this example behaves the same as the example
//! `EventLoop` in the `examples/support` module. It has been inlined in order to provide an
//! example that does not depend on the `support` module.

pub mod feature {

    use gui_node;
    use build;
    use Node;

    use std::rc::Rc;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ops::DerefMut;

    use conrod;
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::glutin;
    use conrod::backend::glium::glium::{DisplayBuild, Surface};
    use std;

    widget_ids! {
        struct Ids {
            canvas,
            text_edit,
            scrollbar,
            name_input_background,
            nodes[]
        }
    }

    struct Params {
        pub node_id: i64,
        pub display_menu: bool,
        pub mouse_x: f64,
        pub mouse_y: f64,
        pub tab_x: f64,
        pub tab_y: f64,
        pub name_input: String,
        pub gui_nodes: Vec<Rc<RefCell<gui_node::GuiNodeData>>>,
        pub connect_node: Option<Rc<RefCell<gui_node::GuiNodeData>>>,
        pub node_map: HashMap<i64, Rc<RefCell<Node>>>,
    }

    pub fn gui() {
        const WIDTH: u32 = 800;
        const HEIGHT: u32 = 600;

        // Build the window.
        let display = glium::glutin::WindowBuilder::new()
            .with_vsync()
            .with_dimensions(WIDTH, HEIGHT)
            .with_title("Hello Conrod!")
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
            gui_nodes: vec![],
            connect_node: None,
            node_map: HashMap::new(),
        };

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
                    ui.handle_event(event);
                    ui_needs_update = true;
                }

                match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                    glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Q)) |
                    glium::glutin::Event::Closed => {
                        if let Some(g_node) = params.gui_nodes.last() {
                            if let Some(node) = params.node_map.get(&g_node.borrow().node_id) {
                                build::pull(node.borrow_mut().deref_mut());
                            }
                            else {
                                println!("failed to find node in node_map");
                            }

                        }
                        else {
                            println!("failed to find last node");
                        }
                        break 'main
                    }
                    glium::glutin::Event::KeyboardInput( glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Tab)) => {
                        params.display_menu = true;
                        params.tab_x = params.mouse_x;
                        params.tab_y = params.mouse_y;
                    }
                    glium::glutin::Event::MouseMoved(x, y) => {
                        params.mouse_x = x as f64;
                        params.mouse_y = y as f64;
                    }
                    _ => {}
                }
            }

            // Instantiate all widgets in the GUI.
            {
                set_ui(ui.set_widgets(), &mut ids, &mut params);
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

    fn set_ui(ref mut ui: conrod::UiCell, ids: &mut Ids, params: &mut Params) {
        use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};
        widget::Canvas::new()
            .color(color::DARK_CHARCOAL)
            .set(ids.canvas, ui);

        if params.display_menu {
            widget::Canvas::new()
                .color(color::RED)
                .pad(5.0)
                .w(200.0)
                .h(30.0)
                .top_left_with_margins_on(ids.canvas, params.tab_y as f64, params.tab_x as f64)
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
                        create_node(params, ui.widget_id_generator());
                        params.name_input = "".to_string();
                        params.display_menu = false;
                    }
                }
            }
        }

        for g_node in params.gui_nodes.iter() {
            let id;
            let node_id;
            {
                let node = g_node.borrow();
                id = node.id;
                node_id = node.node_id;
            }
            for event in gui_node::GuiNode::new(g_node.clone())
                    .parent(ids.canvas)
                    .set(id, ui) {
                match event {
                    gui_node::Event::None => {}
                    ref e => {
                        println!("{:?} {:?}", e, node_id);
                    }
                }
                match event {
                    gui_node::Event::ConnectOutput => {
                        params.connect_node = Some(g_node.clone());
                    }
                    gui_node::Event::ConnectInput => {
                        // conrod::input::global::Global&
                        let global = ui.global_input();
                        let ref state = global.current;
                        let mut nnn = None;
                        println!("Looking for node");
                        for n in params.gui_nodes.iter() {
                            if state
                                   .widget_under_mouse
                                   .map_or(false, |node_index| node_index == n.borrow().id) {
                                nnn = Some(n.clone());
                                println!("Found index");
                            }
                        }
                        // println!("Under mouse {:?}", state.widget_under_mouse);
                        match nnn {
                            Some(ref node) => {
                                println!("Connecting node");
                                let mut nn = node.borrow();
                                build::connect(node_id,
                                               None,
                                               nn.node_id,
                                               Some(1),
                                               &params.node_map);
                            }
                            None => {}
                        }
                    }
                    _ => {}
                }
            }

        }

        widget::Scrollbar::y_axis(ids.canvas)
            .auto_hide(true)
            .set(ids.scrollbar, ui);
    }

    fn create_node(params: &mut Params, mut generator: conrod::widget::id::Generator) {
        let id = params.node_id + 1;
        params.node_id = params.node_id + 1;
        let maybe_node = build::build(id, params.name_input.clone());
        if let Some(node) = maybe_node {
            let g_node = Rc::new(RefCell::new(gui_node::GuiNodeData {
                                                  id: generator.next(),
                                                  node_id: id,
                                                  label: params.name_input.clone(),
                                                  x: params.tab_x,
                                                  y: params.tab_y,
                                                  origin_x: params.tab_x,
                                                  origin_y: params.tab_y,
                                                  mode: gui_node::Mode::None,
                                              }));

            params.node_map.insert(id, node.clone());
            params.gui_nodes.push(g_node);
        }
    }

}
