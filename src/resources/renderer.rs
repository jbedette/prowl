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

use crate::data::{
    Vector2,
};

pub struct RendererResource {
    size: Vector2,
    root: Root,
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
        }
        self.root.put_char(x, y, character, BackgroundFlag::None)
    }

    pub fn prepare(&mut self) {
        let r = &mut self.root;
        if r.window_closed() {
            // quit somehow?
        }
        r.set_default_foreground(WHITE);
        r.clear();
    }

    pub fn flush(&mut self) {
        self.root.flush();
    }
}

impl Default for RendererResource {
    fn default() -> Self {
        Self {
            size: Vector2::new(80, 50),
            root: Root::initializer()
                .font("terminal16x16_gs_ro.png", FontLayout::Tcod)
                .font_type(FontType::Greyscale)
                .size(80, 50)
                .title("PROWL")
                .init(),
        }
    }
}
