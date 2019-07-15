use specs::{
    System,
    WriteStorage,
    ReadStorage,
    Read,
    Write,
    Join
};
use crate::components::{
    Position,
    pending_actions::{
        PendingActions,
        Action,
    },
    Player,
};
use crate::resources::{
    RendererResource,
    Quit,
    input::{
        InputCode,
        UserInput
    },
    console::{
        Console,
        LogLevel,
        Log
    }
};

#[derive(Default)]
pub struct UserInputSystem;

impl<'a> System<'a> for UserInputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, PendingActions>,
        // Read<'a, UserInput>,
        Write<'a, Quit>,
        Write<'a, Console>,
        // TODO get this out of here...
        specs::Write<'a, RendererResource>,
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            players,
            mut positions,
            mut pending_actionses,
            // input,
            mut quit,
            mut console,
            mut renderer,
        ) = data;

        for (
                _player,
                position,
                // pending_actions
            ) in (
                &players,
                &mut positions,
                // &mut pending_actionses,
            ).join() {
            // for some reason input has to be dereferenced.
            // presumably any resource with methods would have to
            // be dereferenced?
            // TODO: input resource may be possible as just Input, need to test
            // so like: `Input::get()`
            // input.get() is blocking, nothing can happen while waiting for
            // input. We will have to re-evaluate input if we want a
            // non-blocking method.

            // NOTE For some reason trying to read pending_actions mutably
            // makes this system non-blocking? WTF
            // delta movement (change to add to position)
            let mut delta = (0, 0);
            let key = UserInput::get(&mut renderer.root);
            (*console).log(Log::new(LogLevel::Debug, "Input Registered"));
            use InputCode::*;
            match key {
                Up => delta.1 = -1,
                Down => delta.1 = 1,
                Left => delta.0 = -1,
                Right => delta.0 = 1,
                Quit => quit.0 = true,
                _ => (),
            }
            if delta != (0, 0) {
                (*console).log(Log::new(LogLevel::Debug, &format!("Player Moved {:?}", delta)));
                // TODO WHY DOESN'T THIS WORK! >:O
                // pending_actions.actions.push(Action::Move { delta });
                position.x += delta.0;
                position.y += delta.1;
            }
        }
    }
}
