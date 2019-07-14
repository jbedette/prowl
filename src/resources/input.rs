use tcod::{
    console::*,
    input::{
        Key,
        KeyCode::*,
    }
};

#[derive(Default)]
pub struct UserInput {}

impl UserInput {
    pub fn get(root: &mut Root) -> InputCode {
        let key = root.wait_for_keypress(true);
        match key {
            Key { code: Up, .. } => return InputCode::Up,
            Key { code: Left, .. } => return InputCode::Left,
            Key { code: Down, .. } => return InputCode::Down,
            Key { code: Right, .. } => return InputCode::Right,
            Key { code: Escape, .. } => return InputCode::Quit,
            _ => InputCode::None,
        }
        // InputCode::None
    }
}

#[derive(Eq, PartialEq)]
pub enum InputCode {
    Up,
    Down,
    Left,
    Right,
    Quit,
    None
}
