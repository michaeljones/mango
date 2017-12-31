use std::rc::Rc;
use std::cell::RefCell;

use conrod::{self, widget, Colorable, Positionable, Widget};

#[derive(Debug, PartialEq)]
pub enum Mode {
    None,
    Drag,
    OutputConnection,
}

#[derive(Debug)]
pub struct GuiNodeData {
    pub id: conrod::widget::id::Id,
    pub parameter_ids: conrod::widget::id::List,
    pub node_id: i64,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub origin_x: f64,
    pub origin_y: f64,
    pub mode: Mode,
}

pub type GuiNodeDataRef = Rc<RefCell<GuiNodeData>>;

#[derive(Clone, WidgetCommon)]
pub struct GuiNode {
    #[conrod(common_builder)] common: widget::CommonBuilder,
    data: GuiNodeDataRef,
    style: Style,
    selected: bool,
}

// We use the `widget_style!` macro to vastly simplify the definition and implementation of the
// widget's associated `Style` type. This generates both a `Style` struct, as well as an
// implementation that automatically retrieves defaults from the provided theme.
//
// See the documenation of the macro for a more details.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {}

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
    pub fn new(data: Rc<RefCell<GuiNodeData>>, selected: bool) -> Self {
        GuiNode {
            common: widget::CommonBuilder::default(),
            data: data,
            style: Style::default(),
            selected: selected,
        }
    }
}

#[derive(Debug)]
pub enum Event {
    None,
    ConnectInput,
    ConnectOutput,
    Click,
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

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        use conrod::{color, Sizeable};
        use conrod::position::{Position, Relative};

        let widget::UpdateArgs {
            id,
            maybe_parent_id,
            state,
            ui,
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
                            data.y = data.origin_y + drag.total_delta_xy[1];
                        }
                    }
                    conrod::event::Widget::Click(click) => {
                        if click.button == conrod::input::state::mouse::Button::Left {
                            output_event = Event::Click;
                        }
                    }
                    conrod::event::Widget::Press(press) => match press.button {
                        conrod::event::Button::Mouse(_, _) => {
                            if press
                                .modifiers
                                .contains(conrod::input::keyboard::ModifierKey::CTRL)
                            {
                                data.mode = Mode::OutputConnection;
                                output_event = Event::ConnectOutput;
                            } else {
                                data.mode = Mode::Drag;
                            }
                        }
                        _ => {}
                    },
                    conrod::event::Widget::Release(release) => match release.button {
                        conrod::event::Button::Mouse(_, _) => {
                            if release
                                .modifiers
                                .contains(conrod::input::keyboard::ModifierKey::CTRL)
                            {
                                output_event = Event::ConnectInput;
                            } else {
                                data.origin_x = data.x;
                                data.origin_y = data.y;
                            }
                        }
                        _ => {}
                    },
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
            .x_position(Position::Relative(
                Relative::Scalar(data.x),
                Some(parent_id),
            ))
            .y_position(Position::Relative(
                Relative::Scalar(data.y),
                Some(parent_id),
            ))
            .w(140.0)
            .h(30.0)
            .flow_right(&[
                (
                    state.ids.input_button,
                    widget::Canvas::new()
                        .graphics_for(id)
                        .parent(state.ids.node)
                        .length(20.0)
                        .rgb(159.0 / 256.0, 168.0 / 256.0, 171.0 / 256.0),
                ),
                (
                    state.ids.body,
                    widget::Canvas::new()
                        .graphics_for(id)
                        .parent(state.ids.node)
                        .length(100.0)
                        .rgb(91.0 / 256.0, 103.0 / 256.0, 107.0 / 256.0)
                        .and_if(self.selected, |w| w.rgb(0.5, 0.5, 0.5)),
                ),
                (
                    state.ids.output_button,
                    widget::Canvas::new()
                        .graphics_for(id)
                        .parent(state.ids.node)
                        .length(20.0)
                        .rgb(113.0 / 256.0, 158.0 / 256.0, 171.0 / 256.0),
                ),
            ])
            .set(state.ids.node, ui);

        widget::primitive::text::Text::new(data.label.as_str())
            .graphics_for(id)
            .parent(state.ids.body)
            .w(100.0)
            .color(color::WHITE)
            .middle_of(state.ids.body)
            .no_line_wrap()
            .set(state.ids.text, ui);

        Some(output_event)
    }
}
