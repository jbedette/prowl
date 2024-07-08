use specs::{Component, VecStorage};
use specs_derive::Component;
use tcod::colors::Color;


#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct CharRenderer {
    pub character: char,
    pub color: Color,
    pub bg_color: Option<Color>,
}

impl CharRenderer {
    pub fn new(character: char, color: Color) -> Self {
        Self { character, color, bg_color: None }
    }

    pub fn with_bg(character: char, color: Color, bg_color: Color) -> Self {
        let bg_color = Some(bg_color);
        Self { character, color, bg_color }
    }

    pub fn ui_border() -> Self {
        CharRenderer::new(' ', Color::new(45, 42, 90))
    }

    pub fn ui_body() -> Self {
        CharRenderer::new(' ', Color::new(12, 24, 32))
    }
}
