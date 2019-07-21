/// This is the input when a UI element is up blocking the rest of the game.
/// TODO Still not really sure how it will work

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
pub struct UserInterfaceSystem;

impl<'a> System<'a> for UserInterfaceSystem {
    type SystemData = (
    );

    /// Responds to user input
    fn run(&mut self, data: Self::SystemData) {
    }
}
