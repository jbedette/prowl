use termion::{
    cursor,
    color,
    style,
    terminal_size,
    raw::IntoRawMode,
    clear,
};

use std::{
    error::Error,
    io::{
        stdout,
        stdin,
        Write,
    }
};


struct EntityRenderer {
    char: char,
    color: color::Rgb,
    position: Vector2,
}

struct Screen {
    dimensions: Vector2,
    top_left: Vector2,
}

impl Screen {
    fn new(d_x: i32, d_y: i32, tl_x: i32, tl_y: i32) -> Self {
        Self {
            dimensions: Vector2::new(d_x, d_y),
            top_left: Vector2::new(tl_x, tl_y)
        }
    }
}

struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct TermionRenderer {
}

impl TermionRenderer {
    pub fn render() {
        let size = terminal_size().unwrap();
        let screen = Screen::new(size.0 as i32, size.1 as i32, 0, 0);
        println!("Size is {:?}", size);
        let player = EntityRenderer {
            char: '@',
            color: color::Rgb(0x00, 0x95, 0xff),
            position: Vector2 { x: 8, y: 8 }
        };
        // let color_string = "#FFFF00";
        // 
        let mut stdout = stdout().into_raw_mode().unwrap();
        writeln!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
        TermionRenderer::render_entity(&mut stdout, player);
    }

    /// Render entity character at screen position...
    fn render_entity<W: Write>(stdout: &mut W, entity_renderer: EntityRenderer) { // , position: Vector2) {
        let (x, y, color, character) = (
            entity_renderer.position.x as u16,
            entity_renderer.position.y as u16,
            entity_renderer.color,
            entity_renderer.char
            );
        writeln!(
           stdout,
           "{}{}@",
           cursor::Goto(x, y),
           color::Fg(color)
            ).unwrap();
    }
}
