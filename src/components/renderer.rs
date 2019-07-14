use specs::{
    Component,
    VecStorage
};
use specs_derive::Component;
use termion::color;
// use std::io::Write;

// use crate::components::position::Position;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CharRenderer {
    pub character: char,
    pub color: color::Rgb,
}

impl CharRenderer {
    pub fn new(character: char, color: color::Rgb) -> Self {
        Self { character, color }
    }
}
