// when creating a panel, this contains the basic values for menu position and color


use crate::components::{CharRenderer};
use specs::prelude::*;
use crate::ui::{
    Panel,
    markers::ConsoleUI,
    markers,
};
use crate::shared::Vector2;
use tcod::colors::Color;

// creates ui panel window in world console view

pub fn init(world: &mut World) {
    world.create_entity()
        .with(Panel::new(
                "Console",
                Vector2::new(50, 10), // positioning magic numbers
                Vector2::new(29, 39), // positioning magic numbers
                CharRenderer::new(' ', Color::new(12, 24, 32)),
                CharRenderer::new(' ', Color::new(45, 42, 90)),
                0))
        .with(ConsoleUI)
        .build();
}

pub fn register(world: &mut World) {
    world.register::<Panel>();
    world.register::<markers::StatusUI>();
    world.register::<markers::InteractiveUI>();
    world.register::<markers::ConsoleUI>();
}