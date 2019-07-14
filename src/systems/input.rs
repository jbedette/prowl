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
    Player,
};
use crate::resources::{
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

pub struct UserInputSystem;

impl<'a> System<'a> for UserInputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        Read<'a, UserInput>,
        Write<'a, Quit>,
        Write<'a, Console>,
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            players,
            mut positions,
            input,
            mut quit,
            mut console
        ) = data;

        for (_, position) in (&players, &mut positions).join() { 
            // for some reason input has to be dereferenced.
            // presumably any resource with methods would have to 
            // be dereferenced.
            // TODO: input resource may be possible as just Input, need to test
            // so like: `Input::get()`
            // input.get() is blocking, nothing can happen while waiting for
            // input. We will have to re-evaluate input if we want a
            // non-blocking method.
            let key: InputCode = (*input).get();
            use InputCode::*;
            match key {
                Up => {
                    position.y -= 1;
                    (*console).log(Log::new(LogLevel::Debug, "Player Moved Up"));
                },
                Down => {
                    position.y += 1;
                    (*console).log(Log::new(LogLevel::Debug, "Player Moved Down"));
                },
                Left => {
                    position.x -= 1;
                    (*console).log(Log::new(LogLevel::Debug, "Player Moved Left"));
                },
                Right => {
                    (*console).log(Log::new(LogLevel::Debug, "Player Moved Right"));
                    position.x += 1;
                },
                Quit => quit.0 = true,
                _ => (),
            }
        }
    }
}
