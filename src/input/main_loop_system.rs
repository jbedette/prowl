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
    game_data::{GameData, StateChangeRequest::{QuitGame, NextTurn}},
    Window,
};
use crate::console::resource::{Log, LogLevel, Console};

use crate::input::tcod_input;

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
            mut console,
            mut game_data,
            mut window,
        ) = data;

        // No player(s)
        if players.is_empty() {
            let key = tcod_input::get(&mut window.root);
            (*console).log(Log::new(LogLevel::Debug, &format!("Player is dead.")));
            use tcod_input::InputCode::*;
            match key {
                // TODO make a way to bring up a menu and stuff
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
            let key = tcod_input::get(&mut window.root);
            use tcod_input::InputCode;
            use InputCode::*;
            match key {
                // Move
                Up => {
                    delta.1 = -1;
                    pending_actions.actions.push(Action::Move { delta });
                    game_data.state_change_request = Some(NextTurn);
                },
                Down => {
                    delta.1 = 1;
                    pending_actions.actions.push(Action::Move { delta });
                    game_data.state_change_request = Some(NextTurn);
                },
                Left => {
                    delta.0 = -1;
                    pending_actions.actions.push(Action::Move { delta });
                    game_data.state_change_request = Some(NextTurn);
                },
                Right => {
                    delta.0 = 1;
                    pending_actions.actions.push(Action::Move { delta });
                    game_data.state_change_request = Some(NextTurn);
                },
                // Quit
                Quit => game_data.state_change_request = Some(QuitGame),
                // Console
                ConsoleSrollUp => console.scroll(-1),
                ConsoleSrollDown => console.scroll(1),
                _ => (),
            }
            // Some inputs increment the current game turn
        }
    }
}
