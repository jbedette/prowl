use specs::{
    Join,
    ReadStorage,
    System,
    // Read,
    Write,
    WriteStorage,
};

use crate::components::{
    pending_actions::{Action, PendingActions},
    Player,
};

use crate::resources::{
    // RendererResource,
    // Quit,
    console::{Console, Log, LogLevel},
    game_data::{GameData, StateChangeRequest::QuitGame},
    Window,
};

#[derive(Default)]
pub struct UserInputSystem;

impl<'a> System<'a> for UserInputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, PendingActions>,
        // Write<'a, Quit>,
        Write<'a, Console>,
        Write<'a, GameData>,
        specs::Write<'a, Window>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            players,
            mut pending_actionses,
            // mut quit,
            mut console,
            mut game_data,
            mut window,
        ) = data;

        if players.is_empty() {
            let key = input::get(&mut window.root);
            (*console).log(Log::new(LogLevel::Debug, &format!("Player is dead.")));
            use input::InputCode::*;
            match key {
                // TODO reset menu
                Quit => game_data.state_change_request = Some(QuitGame),
                _ => (),
            }
            return;
        }
        for (_player, pending_actions) in (&players, &mut pending_actionses).join() {
            // delta movement (change to add to position)
            let mut delta = (0, 0);
            let key = input::get(&mut window.root);
            (*console).log(Log::new(LogLevel::Debug, "Input Registered"));
            use input::InputCode::*;
            match key {
                Up => delta.1 = -1,
                Down => delta.1 = 1,
                Left => delta.0 = -1,
                Right => delta.0 = 1,
                Quit => game_data.state_change_request = Some(QuitGame),
                ConsoleSrollUp => console.y_offset += 1,
                ConsoleSrollDown => console.y_offset -= 1,
                _ => (),
            }
            if delta != (0, 0) {
                pending_actions.actions.push(Action::Move { delta });
            }
        }
    }
}

// Contains input logic.
mod input {
    use tcod::{
        console::*,
        input::{Key, KeyCode::*},
    };

    pub fn get(root: &mut Root) -> InputCode {
        let key = root.wait_for_keypress(true);
        match key {
            /*
            Key { code: Up, .. } => return InputCode::Up,
            Key { code: Left, .. } => return InputCode::Left,
            Key { code: Down, .. } => return InputCode::Down,
            Key { code: Right, .. } => return InputCode::Right,
            */
            Key { code: Char, .. } => {
                match key.printable {
                    'w' => InputCode::Up,
                    'a' => InputCode::Left,
                    's' => InputCode::Down,
                    'd' => InputCode::Right,
                    'k' => InputCode::ConsoleSrollUp,
                    'j' => InputCode::ConsoleSrollDown,
                    _ => InputCode::None,
                }
            },
            Key { code: Escape, .. } => InputCode::Quit,
            _ => InputCode::None,
        }
    }

    #[derive(Eq, PartialEq)]
    pub enum InputCode {
        Up,
        Down,
        Left,
        Right,
        ConsoleSrollUp,
        ConsoleSrollDown,
        Quit,
        None,
    }
}
