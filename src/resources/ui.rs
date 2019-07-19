#![allow(dead_code)]

use crate::shared::Vector2;

#[derive(Default)]
pub struct UIPanel {
    pub top_left: Vector2,
    pub bounds: Vector2,
    border: char,
    contents: String,
}

impl UIPanel {
    fn new(
        top_left: Vector2,
        bounds: Vector2,
        border: char,
        contents: &str,
    ) -> Self {
        let contents = String::from(contents);
        Self {
            top_left,
            bounds,
            border,
            contents,
        }
    }

    fn append_content(&mut self, content: &str) {
        self.contents = format!("{}{}", self.contents, content);
    }

    fn replace_content(&mut self, content: &str) {
        self.contents = String::from(content);
    }
}

pub struct UI {
    pub panels: Vec<UIPanel>,
}

impl Default for UI {
    fn default() -> Self {
        let panel = UIPanel::new(
            Vector2::new(2,2),
            Vector2::new(20,20),
            'X',
            "WOWEE"
            );
        Self {
            panels: vec!(panel),
        }
    }
}
