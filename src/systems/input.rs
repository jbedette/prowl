use specs::{
    Join,
    ReadStorage,
    System,
    Write,
    WriteStorage,
};

use crate::components::{
    pending_actions::{Action, PendingActions},
    markers::Player,
};

use crate::resources::{
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
        Write<'a, Console>,
        Write<'a, GameData>,
        specs::Write<'a, Window>,
    );

    /// Responds to user input
    fn run(&mut self, data: Self::SystemData) {
        let (
            players,
            mut pending_actionses,
            // mut quit,
            mut console,
            mut game_data,
            mut window,
        ) = data;

        // No player(s)
        if players.is_empty() {
            let key = input::get(&mut window.root);
            (*console).log(Log::new(LogLevel::Debug, &format!("Player is dead.")));
            use input::InputCode::*;
            match key {
                // TODO reset menu and stuff
                Quit => game_data.state_change_request = Some(QuitGame),
                _ => (),
            }
            return;
        }
        // 1 or more players
        for (_player, pending_actions) in
                (&players, &mut pending_actionses).join() {
            // Has pending_action, can't take further input
            if !pending_actions.actions.is_empty() { continue; }
            // Delta movement (change to add to position)
            let mut delta = (0, 0);
            // Get user input
            let key = input::get(&mut window.root);
            (*console).log(Log::new(LogLevel::Debug, "Input Registered"));
            use input::InputCode;
            use InputCode::*;
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

            // Some inputs increment the current game turn
            // Others (like UI) don't.
            let mut increment_turn = false;
            if delta != (0, 0) {
                pending_actions.actions.push(Action::Move { delta });
                increment_turn = true;
            }

            // Triggers a turn
            if increment_turn { game_data.current_turn += 1; }
        }
    }
}

/// Polls TCOD for keyboard input. Blocks.
// TODO wtf Non-blocking input keeps repeating after key release.
mod input {
    use tcod::{
        console::*,
        input::{
            Key,
            KeyCode::*,
        },
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
        } else { InputCode::None }
    }

    // Maps a pressed key to an input code.
    // This makes it ez to have 2 keys do the same thing
    fn key_to_input(key: Key) -> InputCode {
        match key {
            Key { code: Char, pressed: true, .. } => {
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
    }
}
