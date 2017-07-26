
use conrod::{self, widget, Widget};

pub struct Background {
    common: widget::CommonBuilder,
    style: Style,
}


// We use the `widget_style!` macro to vastly simplify the definition and implementation of the
// widget's associated `Style` type. This generates both a `Style` struct, as well as an
// implementation that automatically retrieves defaults from the provided theme.
//
// See the documenation of the macro for a more details.
widget_style! {
    style Style {
    }
}

// We'll create the widget using a `Circle` widget and a `Text` widget for its label.
//
// Here is where we generate the type that will produce these identifiers.
widget_ids! {
    struct Ids {
        canvas,
    }
}

pub struct State {
    ids: Ids,
}

impl Background {
    pub fn new() -> Self {
        Background {
            common: widget::CommonBuilder::new(),
            style: Style::new(),
        }
    }
}

#[derive(Debug)]
pub enum Event {
    None,
    Click,
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl Widget for Background {
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
        use conrod::{color, Colorable};

        let widget::UpdateArgs {
            id,
            maybe_parent_id,
            state,
            mut ui,
            ..
        } = args;

        let mut output_event = Event::None;

        {
            let input = ui.widget_input(id);

            for event in input.events() {
                match event {
                    conrod::event::Widget::Click(click) => {
                        if click.button == conrod::input::state::mouse::Button::Left {
                            output_event = Event::Click;
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
            .color(color::DARK_CHARCOAL)
            .set(state.ids.canvas, ui);

        Some(output_event)
    }
}
