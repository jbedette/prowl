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
    }
};

pub struct UserInputSystem;

impl<'a> System<'a> for UserInputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        Read<'a, UserInput>,
        Write<'a, Quit>
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            players,
            mut positions,
            input,
            mut quit
        ) = data;

        for (_, position) in (&players, &mut positions).join() { 
            // for some reason input has to be dereferenced.
            // presumably any resource with methods would have to 
            // be dereferenced.
            // TODO: input resource may be possible as just Input,
            // need to test... so Input::get().
            let key: InputCode = (*input).get();
            use InputCode::*;
            let mut input = true;
            match key {
                Up => position.y -= 1,
                Down => position.y += 1,
                Left => position.x -= 1,
                Right => position.x += 1,
                Quit => quit.0 = true,
                _ => input = false,
            }
            if input { break; }
        }
    }
}
