use std::io::{
    stdout,
    Write,
};

use termion::{
    cursor,
    color,
    style,
    clear,
    raw::{
        IntoRawMode,
        RawTerminal,
    },
};


pub struct RendererResource {
    stdout: RawTerminal<std::io::Stdout>,
}

impl Default for RendererResource {
    fn default() -> Self {
        RendererResource::new()
    }
}

impl RendererResource {
    pub fn new() -> Self {
        Self {
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    /// Renders a character at a location
    pub fn put_char(&mut self, character: char, color: color::Rgb, x: u16, y: u16) {
        writeln!(
           self.stdout,
           "{}{}{}",
           cursor::Goto(x, y),
           color::Fg(color),
           character
            ).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        writeln!(self.stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    }
}

impl Drop for RendererResource {
    fn drop(&mut self) {
        /*
        self.clear();
        writeln!(
            self.stdout,
            "{}{}{}",
            style::Reset,
            cursor::Goto(1,1),
            cursor::Show,
        ).unwrap();
        */
    }
}
