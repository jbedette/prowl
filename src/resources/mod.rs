// resources module is largely taken from tcod docs
// manages actual display of game

pub mod game_data;
pub mod window;
pub mod init;

pub use init::register;

pub use window::TCODWindow as Window;

use game_data::GameData;
