// Menu panel
//  - opens a panel 
//  - takes menu title from object interacted with
//  - manages text displays with a widget system

use crate::{components::CharRenderer, shared::Vector2};
use specs::{Component, VecStorage};
use specs_derive::Component;


// Panel Component
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Panel {
    pub title: String,
    pub position: Vector2,
    pub bounds: Vector2,
    pub border: CharRenderer,
    pub background: CharRenderer,
    pub widgets: Vec<Widget>,
    pub id: i32,
}

impl Panel {
    pub fn new(
        title: &str,
        position: Vector2,
        bounds: Vector2,
        border: CharRenderer,
        background: CharRenderer,
        id: i32
    ) -> Self {
        let title = String::from(title);
        Self {
            title,
            position,
            bounds,
            border,
            background,
            widgets: vec![],
            id
        }
    }
}

#[derive(Debug)]
pub enum Widget {
    Label { text: String },
    TextBox { text: String },
}

// Panel widget takes passed in options vec of strings
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
    #[allow(dead_code)]
    pub fn menu(text: &str, opts: Vec<String>) -> Self {
        let mut text = format! {"{}{}",String::from(text),"\nWould you like to:"};
        for opt in opts.iter().enumerate() {
            text = format! {"{}\n{}){}", text, opt.0+1,opt.1};
        }
        Widget::TextBox { text }
    }
}
