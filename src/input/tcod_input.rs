/// Polls TCOD for keyboard input. Blocks.
// TODO wtf Non-blocking input keeps repeating after key release.
use tcod::{
    console::*,
    input::{Key, KeyCode::*},
};

// Gets keyboard input, returns an 'InputCode'
pub fn get(root: &mut Root) -> InputCode {
    get_blocking(root)
}

// Gets user input. Blocks further execution.
fn get_blocking(root: &mut Root) -> InputCode {
    let key = root.wait_for_keypress(true);
    key_to_input(key)
}

// TODO I cannot get this to work.
#[allow(dead_code)]
fn get_non_blocking(root: &mut Root) -> InputCode {
    let key = root.check_for_keypress(tcod::input::KeyPressFlags::all());
    if let Some(key) = key {
        key_to_input(key)
    } else {
        InputCode::None
    }
}

// Maps a pressed key to an input code.
// This makes it ez to have 2 keys do the same thing
fn key_to_input(key: Key) -> InputCode {
    match key {
        Key {
            code: Char,
            pressed: true,
            ..
        } => match key.printable {
            'w' => InputCode::Up,
            'a' => InputCode::Left,
            's' => InputCode::Down,
            'd' => InputCode::Right,
            'k' => InputCode::ConsoleSrollUp,
            'j' => InputCode::ConsoleSrollDown,
            _ => InputCode::None,
        },
        Key {
            code: Number1,
            pressed: true,
            ..
        } => InputCode::One,
        Key {
            code: Number2,
            pressed: true,
            ..
        } => InputCode::Two,
        Key {
            code: Number3,
            pressed: true,
            ..
        } => InputCode::Three,
        Key {
            code: Number4,
            pressed: true,
            ..
        } => InputCode::Four,
        Key {
            code: Number5,
            pressed: true,
            ..
        } => InputCode::Five,
        Key { code: Escape, .. } => InputCode::Quit,
        _ => InputCode::None,
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum InputCode {
    Up,
    Down,
    Left,
    Right,
    ConsoleSrollUp,
    ConsoleSrollDown,
    Quit,
    None,
    One,
    Two,
    Three,
    Four,
    Five,
}
