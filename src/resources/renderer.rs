use tcod;
use tcod::{
    colors::*,
    console::*,
    /*
    console::{
        Root,
        // RootInitializer
        FontType,
        FontLayout,
        *
    }
    */
};

use crate::shared::{
    Vector2,
};

pub struct RendererResource {
    size: Vector2,
    pub root: Root,
}

impl RendererResource {
    pub fn new() -> Self {
        Self::default()
    }

    /*
    fn initialize() {
    }
    */

    pub fn put_char(&mut self, screen_position: Vector2, character: char) {
        let (x, y) = screen_position.to_tuple();
        if x < 0 || y < 0 {
            println!("X OR Y LESS THAN 0 -> x: {}, y: {}, char: {}", x, y, character);
            return
        }
        self.root.put_char(x, y, character, BackgroundFlag::None)
    }

    pub fn prepare(&mut self) {
        let r = &mut self.root;
        if r.window_closed() {
            // quit somehow?
            panic!()
        }
        r.set_default_foreground(WHITE);
        r.clear();
    }

    pub fn flush(&mut self) {
        self.root.flush();
    }

    pub fn put_window(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let border = '+';
      for x in (x1 + 1)..(x2 - 1) {
        for y in (y1 + 1)..(y2 - 1) {
          self.put_char(Vector2::new(x, y), ' ');
        }
      }
      for x in x1..x2 {
          self.put_char(Vector2::new(x, y1), border);
          self.put_char(Vector2::new(x, y2), border);
      }
      for y in y1..y2 {
          self.put_char(Vector2::new(x1, y), border);
          self.put_char(Vector2::new(x2, y), border);
      }
    }

    pub fn get_bounds(&self) -> Vector2 {
        return self.size;

    }

    pub fn put_text(&mut self, position: Vector2, string: &str) {
        self.root.print_ex(
            position.x,
            position.y,
            BackgroundFlag::None,
            TextAlignment::Left,
            string
            );
    }
}

impl Default for RendererResource {
    fn default() -> Self {
        tcod::system::set_fps(20);
        Self {
            size: Vector2::new(80, 50),
            root: Root::initializer()
                .font("terminal16x16_gs_ro.png", FontLayout::AsciiInRow)
                // .font("arial10x10.png", FontLayout::Tcod)
                .font_type(FontType::Greyscale)
                .size(80, 50)
                .title("PROWL")
                .init(),
        }
    }

}
