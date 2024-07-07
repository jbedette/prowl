// shared module contains custom functions and data types used throughout project

mod application_root_dir;
mod vector2;
pub mod random;
mod normalize;

pub use application_root_dir::application_root_dir;
pub use vector2::Vector2;
pub use normalize::normalize_vec;
