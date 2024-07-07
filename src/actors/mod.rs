//  The actors modules contains definintions for all
//  entities that have enough agency in the game to 
//  affect the systems and resources 

pub mod islands;
pub mod populations;
pub mod ships;
pub mod player;
pub mod init;


pub use islands::island_component::Island;
// pub use populations::population_component::Population;
pub use init::register;

