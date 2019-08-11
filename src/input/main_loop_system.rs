use specs::{Join, ReadStorage, System, Write, WriteStorage};

use crate::components::{
    markers::Player,
    pending_actions::{Action, PendingActions},
};

use crate::console::resource::{Console, Log, LogLevel};
use crate::resources::{
    game_data::{
        GameData,
        StateChangeRequest::{NextTurn, QuitGame},
    },
    Window,
};

use crate::input::tcod_input;

use crate::shared::Vector2;

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
        let (players, mut pending_actionses, mut console, mut game_data, mut window) = data;

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
        for (_player, pending_actions) in (&players, &mut pending_actionses).join() {
            // Has pending_action, can't take further input
            if !pending_actions.actions.is_empty() {
                continue;
            }
            // Delta movement (change to add to position)
            let delta;
            // Get user input
            let key = tcod_input::get(&mut window.root);
            use tcod_input::InputCode;
            use InputCode::*;
            match key {
                // Move
                Up => {
                    delta = Vector2::north();
                    pending_actions.actions.push(Action::Move { delta });
                    game_data.state_change_request = Some(NextTurn);
                }
                Down => {
                    delta = Vector2::south();
                    pending_actions.actions.push(Action::Move { delta });
                    game_data.state_change_request = Some(NextTurn);
                }
                Left => {
                    delta = Vector2::west();
                    pending_actions.actions.push(Action::Move { delta });
                    game_data.state_change_request = Some(NextTurn);
                }
                Right => {
                    delta = Vector2::east();
                    pending_actions.actions.push(Action::Move { delta });
                    game_data.state_change_request = Some(NextTurn);
                }
                One => {
                    println!("1");
                    game_data.state_change_request = Some(NextTurn);
                }
                Two => {
                    println!("2");
                }
                Three => {
                    println!("3");
                }
                Four => {
                    println!("4");
                }
                Five => {
                    println!("5");
                }
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
