use specs::{
    Write,
    System,
};
pub struct OnTurnSystem;
#[allow(unused_imports)]
use crate::console::resource::{Console, Log, LogLevel};

impl<'a> System<'a> for OnTurnSystem {
    type SystemData = (Write<'a, Console>);

    fn run(&mut self, data: Self::SystemData) {
        let mut console = data;
        // console.logs = vec![];
        console.log(Log::new(
                LogLevel::Game,
                "Time has passed...",
                ));
        console.y_offset = console.logs.len() as i32 - 1;

    }
}
