use specs::{Component, VecStorage};
use specs_derive::Component;
use crate::{
    shared::Vector2,
    components::CharRenderer,
};

/// Panel Component
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Panel {
    pub title: String,
    pub position: Vector2,
    pub bounds: Vector2,
    pub border: CharRenderer,
    pub background: CharRenderer,
    pub widgets: Vec<Widget>
}

impl Panel {
    pub fn new(
        title: &str,
        position: Vector2,
        bounds: Vector2,
        border: CharRenderer,
        background: CharRenderer,
        ) -> Self {
        let title = String::from(title);
        Self {
            title,
            position,
            bounds,
            border,
            background,
            widgets: vec![],
        }
    }
}

#[derive(Debug)]
pub enum Widget {
    Label {
        text: String,
    },
    TextBox {
        text: String,
    },
}

impl Widget {
    // Single line, full line
    pub fn label(text: &str) -> Self {
        let text = String::from(text);
        Widget::Label { text }
    }
    // Multi-line, full line, to-fit
    pub fn text_box(text: &str) -> Self {
        let text = String::from(text);
        Widget::TextBox { text }
    }
}
