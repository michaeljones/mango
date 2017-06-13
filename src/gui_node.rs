
use std::rc::Rc;
use std::cell::RefCell;

use conrod::{self, widget, Colorable, Dimensions, Labelable, Point, Positionable, Widget};

#[derive(PartialEq)]
pub enum Mode {
    None,
    Drag,
    OutputConnection,
}

pub struct GuiNodeData {
    pub id: conrod::widget::id::Id,
    pub node_id: i64,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub origin_x: f64,
    pub origin_y: f64,
    pub mode: Mode,
}

pub struct GuiNode {
    common: widget::CommonBuilder,
    data: Rc<RefCell<GuiNodeData>>,
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
        input_button,
        output_button,
        body,
    }
}

pub struct State {
    ids: Ids,
}

impl GuiNode {
    pub fn new(data: Rc<RefCell<GuiNodeData>>) -> Self {
        GuiNode {
            common: widget::CommonBuilder::new(),
            data: data,
            style: Style::new(),
            enabled: true,
        }
    }

    pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
}

#[derive(Debug)]
pub enum Event {
    None,
    ConnectInput,
    ConnectOutput,
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
    type Event = Option<Event>;

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

        let mut data = self.data.borrow_mut();
        let mut output_event = Event::None;

        {
            let input = ui.widget_input(id);

            for event in input.events() {
                match event {
                    conrod::event::Widget::Drag(drag) => {
                        if data.mode == Mode::Drag {
                            data.x = data.origin_x + drag.total_delta_xy[0];
                            data.y = data.origin_y - drag.total_delta_xy[1];
                        }
                    }
                    conrod::event::Widget::Press(press) => {
                        match press.button {
                            conrod::event::Button::Mouse(_, point) => {
                                if press.modifiers.contains(conrod::input::keyboard::CTRL) {
                                    data.mode = Mode::OutputConnection;
                                    output_event = Event::ConnectOutput;
                                } else {
                                    data.mode = Mode::Drag;
                                }
                            }
                            _ => {}
                        }
                    }
                    conrod::event::Widget::Release(release) => {
                        match release.button {
                            conrod::event::Button::Mouse(_, point) => {
                                if release.modifiers.contains(conrod::input::keyboard::CTRL) {
                                    output_event = Event::ConnectInput;
                                } else {
                                    data.origin_x = data.x;
                                    data.origin_y = data.y;
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        let parent_id = maybe_parent_id.unwrap_or(id);

        // Draw the node
        widget::Canvas::new()
            .graphics_for(id)
            .parent(parent_id)
            .top_left_with_margins_on(parent_id, data.y as f64, data.x as f64)
            .w(140.0)
            .h(30.0)
            .flow_right(&[(state.ids.input_button,
                           widget::Canvas::new()
                               .graphics_for(id)
                               .parent(state.ids.node)
                               .w(20.0)
                               .h(30.0)
                               .color(color::RED)),

                          (state.ids.body,
                           widget::Canvas::new()
                               .graphics_for(id)
                               .parent(state.ids.node)
                               .w(100.0)
                               .h(30.0)
                               .color(color::BLACK)),

                          (state.ids.output_button,
                           widget::Canvas::new()
                               .graphics_for(id)
                               .parent(state.ids.node)
                               .w(20.0)
                               .h(30.0)
                               .color(color::BLUE))])
            .color(color::BLACK)
            .set(state.ids.node, ui);

        widget::primitive::text::Text::new(data.label.as_str())
            .graphics_for(id)
            .parent(state.ids.body)
            .color(color::WHITE)
            .middle_of(state.ids.body)
            .set(state.ids.text, ui);

        Some(output_event)
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
