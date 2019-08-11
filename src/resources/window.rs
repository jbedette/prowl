#![allow(unused)]
// Approach to interacting with TCOD inspired by
// https://gist.github.com/AndrewJakubowicz/3cc636d10bd1dfe1abd08b990472b822
use crate::shared::{
    application_root_dir, 
    Vector2};
use tcod::{console::*, input::KeyCode};

/// Contains the application window.
pub struct TCODWindow {
    pub quit_requested: bool,
    pub key_press: Option<KeyCode>,
    pub root: Root,
    pub size: Vector2,
}

impl TCODWindow {
    // Create a new window, initialize TCOD
    pub fn new(title: &str, fps: i32, size: Vector2, font_path: &str) -> Self {
        tcod::system::set_fps(fps);
        let root = Root::initializer()
            .font(
                application_root_dir().unwrap().join(font_path),
                FontLayout::AsciiInRow,
            )
            .font_type(FontType::Greyscale)
            .size(size.x, size.y)
            .title(title)
            .init();
        Self {
            quit_requested: false,
            key_press: None,
            size,
            root,
        }
    }

    // Draws a TCOD console to the screen.
    pub fn blit(&mut self, console: &Console) {
        tcod::console::blit(
            &console,
            (0, 0),
            self.size.to_tuple(),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
        self.root.flush();
    }
}

impl Default for TCODWindow {
    fn default() -> Self {
        Self::new(
            "DEFAULT",
            20,
            Vector2::new(80, 50),
            "terminal16x16_gs_ro.png",
        )
    }
}
