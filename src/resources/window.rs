#![allow(unused)]
// Approach to interacting with TCOD inspired by
// https://gist.github.com/AndrewJakubowicz/3cc636d10bd1dfe1abd08b990472b822
use tcod::{
    console::*,
    input::KeyCode,
};
use crate::shared::{
    Vector2,
    application_root_dir,
};

/// Contains the application window.
pub struct TCODWindow {
    pub quit_requested: bool,
    pub key_press: Option<KeyCode>,
    pub root: Root,
    pub size: Vector2,
    // pub consoles: Option<RenderingConsoles>,
}

pub struct RenderingConsoles {
    pub main: Offscreen,
    pub map: Offscreen,
    pub characters: Offscreen,
    pub ui: Offscreen,
}

impl TCODWindow {
    // Create a new window, initialize TCOD
    pub fn new(
        title: &str,
        fps: i32,
        size: Vector2,
        font_path: &str,
    ) -> Self {
        tcod::system::set_fps(fps);
        Self {
            quit_requested: false,
            key_press: None,
            size: size,
            root: Root::initializer()
                .font(application_root_dir()
                     .unwrap().join(font_path),
                FontLayout::AsciiInRow)
                .font_type(FontType::Greyscale)
                .size(size.x, size.y)
                .title(title)
                .init(),
                /*
            consoles: {
                main: Offscreen = Offscreen::new(size.x, size.y),
                map: Offscreen::new(size.x, size.y),
                characters: Offscreen::new(size.x, size.y),
                ui: Offscreen::new(size.x, size.y),
            }
            */
        }
    }

    /*
    pub fn blit(&self) {
        tcod::console::blit(
            &mut self.console.main,
            (0, 0)
            (self.size.x, self.size.y),
            &mut self.root,
            (0, 0),
            1.0,
            1.0
        );
    }
    */
}

impl Default for TCODWindow {
    fn default() -> Self {
        Self::new(
            "DEFAULT",
            20,
            Vector2::new(80, 50),
            "terminal16x16_gs_ro.png")
    }
}
