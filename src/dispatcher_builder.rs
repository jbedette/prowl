use crate::systems::{
    AISystem,
    DeathSystem,
    ExecuteActionSystem,
};
use crate::input::{
    main_loop_system::UserInputSystem,
};
use crate::renderer::{
    rendering_system::RenderingSystem,
};
use crate::ui::ui_systems::{
    // InteractiveUISystem,
    StatusWindowSystem,
    ConsoleWindowSystem,
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
        .with(StatusWindowSystem, "status", &[])
        .with(ConsoleWindowSystem, "console_window", &[])
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
        .with(StatusWindowSystem, "status", &[])
        .with(ConsoleWindowSystem, "console_window", &[])
        .build()
}

pub fn ui_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with_thread_local(RenderingSystem)
        .build()
}
