
use std::rc::Rc;
use std::cell::RefCell;
use conrod::{self, widget, Colorable, Dimensions, Labelable, Point, Positionable, Widget};

pub struct GuiNodeData {
    pub id: conrod::widget::id::Id,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub origin_x: f64,
    pub origin_y: f64,
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

        let mut data = self.data.borrow_mut();

        let (color, event) = {
            let input = ui.widget_input(id);

            for event in input.events() {
                match event {
                    conrod::event::Widget::Drag(drag) => {
                        data.x = data.origin_x + drag.total_delta_xy[0];
                        data.y = data.origin_y - drag.total_delta_xy[1];
                    }
                    conrod::event::Widget::Release(press) => {
                        match press.button {
                            conrod::event::Button::Mouse(_, point) => {
                                data.origin_x = data.x;
                                data.origin_y = data.y;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

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
            .top_left_with_margins_on(parent_id, data.y as f64, data.x as f64)
            .w(100.0)
            .h(30.0)
            .color(color::BLACK)
            .set(state.ids.node, ui);

        widget::primitive::text::Text::new(data.label.as_str())
            .graphics_for(id)
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
