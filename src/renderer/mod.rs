use termion::{
    cursor,
    color,
    style,
    terminal_size,
    raw::IntoRawMode,
    clear,
    input::TermRead,
    event::Key,
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

#[derive(Clone)]
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
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        TermionRenderer::initialize(&mut stdout);

        let size = terminal_size().unwrap();
        // let screen = Screen::new(size.0 as i32, size.1 as i32, 0, 0);
        let mut player = EntityRenderer {
            char: '@',
            color: color::Rgb(0x00, 0x95, 0xff),
            position: Vector2 { x: 8, y: 8 },
        };

        stdout.flush().unwrap();
        for c in stdin.keys() {
            TermionRenderer::clear_at(&mut stdout, &player.position);
            match c.unwrap() {
                Key::Char('h') | Key::Char('a') => player.position.x -= 1,
                Key::Char('j') | Key::Char('s') => player.position.y += 1,
                Key::Char('k') | Key::Char('w') => player.position.y -= 1,
                Key::Char('l') | Key::Char('d') => player.position.x += 1,
                Key::Char('q') => break,
                _ => ()
            }
            TermionRenderer::render_entity(&mut stdout, &player);
            stdout.flush().unwrap();
        }

        TermionRenderer::cleanup(&mut stdout);
    }

    fn initialize<W: Write>(stdout: &mut W) {
        writeln!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    }

    fn cleanup<W:Write>(stdout: &mut W) {
        writeln!(
            stdout,
            "{}{}{}",
            style::Reset,
            cursor::Goto(1, 1),
            cursor::Show,
        ).unwrap();
    }

    /// Render entity character at screen position...
    fn render_entity<W: Write>(stdout: &mut W, entity_renderer: &EntityRenderer) { // , position: Vector2) {
        let (x, y, color, character) = (
            entity_renderer.position.x as u16,
            entity_renderer.position.y as u16,
            entity_renderer.color,
            entity_renderer.char
            );
        writeln!(
           stdout,
           "{}{}{}",
           cursor::Goto(x, y),
           color::Fg(color),
           character
            ).unwrap();
    }

    fn clear_at<W: Write>(stdout: &mut W, position: &Vector2) {
        let (x, y) = (
            position.x as u16,
            position.y as u16
            );
        writeln!(
            stdout,
            "{} ",
            cursor::Goto(x, y)
        ).unwrap();
    }
}
