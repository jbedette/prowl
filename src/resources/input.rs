use std::{
    io::{
        stdin,
        Stdin,
    }
};
use termion::{
    input::TermRead,
    event::Key,
};

#[derive(Debug)]
pub struct UserInput {
    stdin: Stdin,
    // stdin: termion::input::Keys<Stdin>
}

impl UserInput {
    pub fn new() -> Self {
        Self { stdin: stdin() }
    }

    pub fn get(&self) -> InputCode {
        let stdin = stdin();
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char('w') => return InputCode::Up,
                Key::Char('a') => return InputCode::Left,
                Key::Char('s') => return InputCode::Down,
                Key::Char('d') => return InputCode::Right,
                Key::Char('q') => return InputCode::Quit,
                _ => ()
            }
        }
        InputCode::None
    }
}

impl Default for UserInput {
    fn default() -> Self {
        Self::new()
    }
}

pub enum InputCode {
    Up,
    Down,
    Left,
    Right,
    Quit,
    None
}
