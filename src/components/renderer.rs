use specs::{
    Component,
    VecStorage
};
use specs_derive::Component;
// use termion::color;
use tcod::{
    colors::Color,
};
// use std::io::Write;

// use crate::components::position::Position;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CharRenderer {
    pub character: char,
    pub color: Color,
}

impl CharRenderer {
    pub fn new(character: char, color: Color) -> Self {
        Self { character, color }
    }
}
