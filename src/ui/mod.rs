// ui module manages the data and interaction displays for the game
//  - it opens when the player runs into a blocking tile
//  - menu blocks the turn system from progressing until it is closed by working with the event channel
//  - in demo, we take the two most recent entities that had something happen to them,
//      because of turn system this will be the two entitites we want to interact


pub mod panel;
pub mod init;
pub mod markers;
pub mod ui_systems;

pub use panel::Panel;
pub use init::register;


