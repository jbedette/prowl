use crate::systems::{
    AISystem,
    DeathSystem,
    ExecuteActionSystem,
    UserInputSystem,
};

use crate::renderer::{
    rendering_system::RenderingSystem,
};

use crate::console;

use specs::{
    DispatcherBuilder,
    Dispatcher
};

// TODO not 'static?
/// initialize systems in the loader state
pub fn setup_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        // rendering must be on local thread
        .with_thread_local(RenderingSystem)
        .build()
}

pub fn input_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with_thread_local(RenderingSystem)
        .with(UserInputSystem, "input", &[])
        .build()
}

pub fn turn_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with_thread_local(RenderingSystem)
        .with(console::on_turn::OnTurnSystem, "console", &[])
        .with(AISystem, "ai", &[])
        .with(ExecuteActionSystem, "execute_actions", &[])
        .with(DeathSystem, "deaths", &[])
        .build()
}
