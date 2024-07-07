// systems module contains the logic for game systems
// many of the commented out lines here represent goals for future development

mod death_system;
// mod rival_system;
// mod print_entity_system;
// mod print_stats_system;
mod ai;
mod execute_actions;
mod interaction_system;
// mod input;
mod food_system;

pub use death_system::DeathSystem;
// pub use rival_system::RivalSystem;
// pub use print_stats_system::PrintStatsSystem;
// pub use print_entity_system::PrintEntitySystem;
pub use ai::AISystem;
pub use execute_actions::ExecuteActionSystem;
pub use interaction_system::InteractionSystem;
pub use food_system::FoodSystem;
