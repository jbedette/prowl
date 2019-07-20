/*
goals
    pop tics up on island, island revolutions
        if nation has more than one island,  nation         loses island
        else new ruler instated
*/
use specs::{Entities, ReadStorage, System, Write};
use crate::resources::console::{Console, Log, LogLevel};
use crate::components::{Island, Population, Ruler, Nation};

pub struct island_ruler_nationSystem;

impl<'a> System<'a> for island_ruler_nationSystem{
    type SystemData = ( 
        ReadStorage<'a, Named>,
        ReadStorage<'a, Population>,
        ReadStorage<'a, Island>,
        ReadStorage<'a, Ruler>,
        ReadStorage<'a, Nation>,
        Entities<'a>,
        Write<'a>, Console,
    );
    
    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
}