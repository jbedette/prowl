use crate::components::{CharRenderer};
use specs::prelude::*;
use crate::ui::{
    Panel,
    markers::ConsoleUI,
};
use crate::shared::Vector2;
use tcod::colors::Color;

pub fn init(world: &mut World) {
    // Makes main UI console window.
    world.create_entity()
        .with(Panel::new(
                "Console",
                Vector2::new(50, 10),
                Vector2::new(29, 39),
                CharRenderer::new(' ', Color::new(12, 24, 32)),
                CharRenderer::new(' ', Color::new(45, 42, 90)),
                ))
        .with(ConsoleUI)
        .build();
}
