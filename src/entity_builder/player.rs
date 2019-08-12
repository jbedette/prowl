/// Builds player ship(s?)
use crate::components::{
        markers::{Player, Ship},
        pending_actions::PendingActions,
        CharRenderer, Health, Money, Named, Position, Weapon,
};
use crate::shared::{random::random_range, Vector2};
use crate::ui::{markers::StatusUI, Panel};
use specs::prelude::*;
use tcod::colors::*;

// TODO what determines parameters ?
pub fn make_player(world: &mut World) {
        let name = "Imona Bote";
        let health = random_range(80, 200) as i64;
        let weapon = random_range(1, 10) as u64;
        let money = random_range(30, 300) as u64;
        let color = Color::new(0x00, 0x95, 0xff);
        world.create_entity()
                .with(Named::new(name))
                .with(Health::new(health, health))
                // does nothing yet
                .with(Weapon::new(weapon))
                // does nothing yet
                .with(Money::new(money))
                .with(Position::new(Vector2::new(30, 30)))
                .with(PendingActions::default())
                // special
                .with(CharRenderer::new('@', color))
                // Stats panel
                .with(Panel::new(
                        "Stats",
                        Vector2::new(50, 1),
                        Vector2::new(29, 10),
                        CharRenderer::ui_body(),
                        CharRenderer::ui_border(),
                        0
                ))
                .with(StatusUI::default())
                // special
                .with(Player::default())
                .with(Ship::default())
                .build();
}
