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

// struct PanelSystem;

#[derive(Debug)]
pub struct Widget {
    position: Vector2,
    bounds: Vector2,
    widget_type: WidgetType
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum WidgetType {
    Label,
    Quantity,
    CurrentOfMax,
    ProgressBar,
}

#[allow(dead_code)]
pub enum BorderType {
    Flat
}
