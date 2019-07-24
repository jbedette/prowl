use crate::systems::{
    AISystem,
    DeathSystem,
    ExecuteActionSystem,
    InteractionSystem,
};
use crate::input::{
    main_loop_system::UserInputSystem,
};
use crate::renderer::{
    rendering_system::RenderingSystem,
};
use crate::ui::ui_systems::{
    InteractiveUISystem,
    StatusWindowSystem,
    ConsoleWindowSystem,
};

use crate::console;

use specs::{
    DispatcherBuilder,
    Dispatcher
};

// TODO for all - can this happen without the 'static lifetimes?
/// Initialize systems in the loader state.
pub fn setup_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        // rendering must be on local thread
        .with_thread_local(RenderingSystem)
        .build()
}

/// Waits for user input.
pub fn input_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with_thread_local(RenderingSystem)
        .with(UserInputSystem, "input", &[])
        .with(StatusWindowSystem, "status", &[])
        .with(ConsoleWindowSystem, "console_window", &[])
        .build()
}

/// Executes a turn.
pub fn turn_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with_thread_local(RenderingSystem)
        .with(console::on_turn::OnTurnSystem, "console", &[])
        .with(AISystem, "ai", &[])
        .with(ExecuteActionSystem, "execute_actions", &[])
        .with(DeathSystem, "deaths", &[])
        .with(StatusWindowSystem, "status", &[])
        .with(InteractionSystem, "interactions", &["execute_actions"])
        .with(ConsoleWindowSystem, "console_window", &[])
        .build()
}

/// Used when UI is blocking, e.g. a menu.
pub fn ui_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with(InteractiveUISystem, "interactive", &[])
        .with_thread_local(RenderingSystem)
        .build()
}

/// Stopgap solution to render after world.maintain() after UI input
pub fn render_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with_thread_local(RenderingSystem)
        .build()
}
