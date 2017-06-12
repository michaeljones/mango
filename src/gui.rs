//! A simple example that demonstrates using conrod within a basic `winit` window loop, using
//! `glium` to render the `conrod::render::Primitives` to screen.
//!
//! Note that the event loop that we create in this example behaves the same as the example
//! `EventLoop` in the `examples/support` module. It has been inlined in order to provide an
//! example that does not depend on the `support` module.

pub mod feature {
    pub mod gui_node {
        use conrod::{self, widget, Colorable, Dimensions, Labelable, Point, Positionable, Widget};

        pub struct GuiNode {
            common: widget::CommonBuilder,
            label: String,
            x: f64,
            y: f64,
            style: Style,
            enabled: bool,
        }

        // We use the `widget_style!` macro to vastly simplify the definition and implementation of the
        // widget's associated `Style` type. This generates both a `Style` struct, as well as an
        // implementation that automatically retrieves defaults from the provided theme.
        //
        // See the documenation of the macro for a more details.
        widget_style! {
            style Style {
                - color: conrod::Color { theme.shape_color }
                - label_color: conrod::Color { theme.label_color }
                - label_font_size: conrod::FontSize { theme.font_size_medium }
                - label_font_id: Option<conrod::text::font::Id> { theme.font_id }
            }
        }

        // We'll create the widget using a `Circle` widget and a `Text` widget for its label.
        //
        // Here is where we generate the type that will produce these identifiers.
        widget_ids! {
            struct Ids {
                node,
                text,
            }
        }

        pub struct State {
            ids: Ids,
        }

        pub fn is_over_circ(circ_center: Point, mouse_point: Point, dim: Dimensions) -> bool {
            // Offset vector from the center of the circle to the mouse.
            let offset = conrod::utils::vec2_sub(mouse_point, circ_center);

            // If the length of the offset vector is less than or equal to the circle's
            // radius, then the mouse is inside the circle. We assume that dim is a square
            // bounding box around the circle, thus 2 * radius == dim[0] == dim[1].
            let distance = (offset[0].powf(2.0) + offset[1].powf(2.0)).sqrt();
            let radius = dim[0] / 2.0;
            distance <= radius
        }

        impl GuiNode {
            pub fn new(label: String, x: f64, y: f64) -> Self {
                GuiNode {
                    common: widget::CommonBuilder::new(),
                    label: label,
                    x: x,
                    y: y,
                    style: Style::new(),
                    enabled: true,
                }
            }

            pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
                self.style.label_font_id = Some(Some(font_id));
                self
            }
        }

        /// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
        /// documentation for more details.
        impl Widget for GuiNode {
            /// The State struct that we defined above.
            type State = State;
            /// The Style struct that we defined using the `widget_style!` macro.
            type Style = Style;
            /// The event produced by instantiating the widget.
            ///
            /// `Some` when clicked, otherwise `None`.
            type Event = Option<()>;

            fn common(&self) -> &widget::CommonBuilder {
                &self.common
            }

            fn common_mut(&mut self) -> &mut widget::CommonBuilder {
                &mut self.common
            }

            fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
                State { ids: Ids::new(id_gen) }
            }

            fn style(&self) -> Self::Style {
                self.style.clone()
            }

            /// Update the state of the button by handling any input that has occurred since the last
            /// update.
            fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
                use conrod::{color, Sizeable};

                let widget::UpdateArgs {
                    id,
                    maybe_parent_id,
                    state,
                    rect,
                    mut ui,
                    style,
                    ..
                } = args;

                let (color, event) = {
                    let input = ui.widget_input(id);

                    // If the button was clicked, produce `Some` event.
                    let event = input.clicks().left().next().map(|_| ());

                    let color = style.color(&ui.theme);
                    let color = input
                        .mouse()
                        .map_or(color,
                                |mouse| if is_over_circ([0.0, 0.0], mouse.rel_xy(), rect.dim()) {
                                    if mouse.buttons.left().is_down() {
                                        color.clicked()
                                    } else {
                                        color.highlighted()
                                    }
                                } else {
                                    color
                                });

                    (color, event)
                };

                let parent_id = maybe_parent_id.unwrap_or(id);

                // Draw the node
                widget::Canvas::new()
                    .graphics_for(id)
                    .parent(parent_id)
                    .top_left_with_margins_on(parent_id, self.y, self.x)
                    .w(100.0)
                    .h(30.0)
                    .color(color::BLACK)
                    .set(state.ids.node, ui);

                widget::primitive::text::Text::new(self.label.as_str())
                    .parent(state.ids.node)
                    .color(color::WHITE)
                    .middle_of(state.ids.node)
                    .set(state.ids.text, ui);

                event
            }
        }

        /// Provide the chainable color() configuration method.
        impl Colorable for GuiNode {
            fn color(mut self, color: conrod::Color) -> Self {
                self.style.color = Some(color);
                self
            }
        }

        /// Provide the chainable label(), label_color(), and label_font_size()
        /// configuration methods.
        impl<'a> Labelable<'a> for GuiNode {
            fn label(mut self, text: &'a str) -> Self {
                self.label = text.to_string();
                self
            }
            fn label_color(mut self, color: conrod::Color) -> Self {
                self.style.label_color = Some(color);
                self
            }
            fn label_font_size(mut self, size: conrod::FontSize) -> Self {
                self.style.label_font_size = Some(size);
                self
            }
        }
    }

    use conrod;
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::glutin;
    use conrod::backend::glium::glium::{DisplayBuild, Surface};
    use std;
    use std::rc::Rc;
    use std::cell::RefCell;

    widget_ids! {
        struct Ids {
            canvas,
            text_edit,
            scrollbar,
            name_input_background,
            nodes[]
        }
    }

    struct GuiNode {
        pub id: conrod::widget::id::Id,
        pub name: String,
        pub x: i32,
        pub y: i32,
    }

    struct Params {
        pub display_menu: bool,
        pub mouse_x: i32,
        pub mouse_y: i32,
        pub tab_x: i32,
        pub tab_y: i32,
        pub name_input: String,
        pub gui_nodes: Vec<Rc<RefCell<GuiNode>>>,
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
            display_menu: false,
            mouse_x: 0,
            mouse_y: 0,
            tab_x: 0,
            tab_y: 0,
            name_input: String::new(),
            gui_nodes: vec![],
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
                    glium::glutin::Event::Closed =>
                        break 'main,
                    glium::glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Tab)) => {
                        params.display_menu = true;
                        params.tab_x = params.mouse_x;
                        params.tab_y = params.mouse_y;
                    }
                    glium::glutin::Event::MouseMoved(x, y) => {
                        params.mouse_x = x;
                        params.mouse_y = y;
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
                .w(100.0)
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
            let node = g_node.borrow_mut();
            gui_node::GuiNode::new(node.name.clone(), node.x as f64, node.y as f64)
                .parent(ids.canvas)
                .set(node.id, ui);
        }

        widget::Scrollbar::y_axis(ids.canvas)
            .auto_hide(true)
            .set(ids.scrollbar, ui);
    }

    fn create_node(params: &mut Params, mut generator: conrod::widget::id::Generator) {
        let g_node = Rc::new(RefCell::new(GuiNode {
                                              id: generator.next(),
                                              name: params.name_input.clone(),
                                              x: params.tab_x,
                                              y: params.tab_y,
                                          }));

        params.gui_nodes.push(g_node);
    }
}
