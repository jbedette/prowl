use crate::components::{CharRenderer};
use specs::prelude::*;
use crate::ui::Panel;
use crate::shared::Vector2;
use tcod::colors::Color;

pub fn init(world: &mut World) {
    // Makes main UI stats window.
    world.create_entity()
        .with(Panel::new(
                "Player Stats",
                Vector2::new(1, 1),
                Vector2::new(14, 6),
                CharRenderer::new(' ', Color::new(12, 24, 32)),
                CharRenderer::new(' ', Color::new(45, 42, 90)),
                ))
        .build();
    // Makes main UI console window.
    world.create_entity()
        .with(Panel::new(
                "Console",
                Vector2::new(44, 29),
                Vector2::new(35, 20),
                CharRenderer::new(' ', Color::new(12, 24, 32)),
                CharRenderer::new(' ', Color::new(45, 42, 90)),
                ))
        .build();
}
