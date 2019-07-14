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
    terminal_size,
};


pub struct RendererResource {
    stdout: RawTerminal<std::io::Stdout>,
    size: (u16, u16)
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
            size: terminal_size().unwrap(),
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

    pub fn put_text(&mut self, text: &str, x: u16, y: u16) {
      let color = color::Rgb(0xbb, 0xbb, 0xbb);
        writeln!(
           self.stdout,
           "{}{}{}",
           cursor::Goto(x, y),
           color::Fg(color),
           text
            ).unwrap();

    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn clear(&mut self) {
        writeln!(self.stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    }

    pub fn prepare(&mut self) {
      self.update_size();
      self.clear();
    }

    fn update_size(&mut self) {
      self.size = terminal_size().unwrap();
    }

    pub fn get_bounds(&self) -> (u16, u16) {
      self.size
    }

    pub fn put_window(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) {
      let color = color::Rgb(0x10, 0x40, 0x30);
      for x in (x1 + 1)..(x2 - 1) {
        for y in (y1 + 1)..(y2 - 1) {
          self.put_char(' ', color, x, y);
        }
      }
      for x in x1..x2 {
        self.put_char('#', color, x, y1);
        self.put_char('#', color, x, y2);
      }
      for y in y1..y2 {
        self.put_char('#', color, x1, y);
        self.put_char('#', color, x2, y);
      }
    }
}

impl Drop for RendererResource {
    fn drop(&mut self) {
        self.clear();
        writeln!(
            self.stdout,
            "{}{}{}",
            style::Reset,
            cursor::Goto(1,1),
            cursor::Show,
        ).unwrap();
    }
}
