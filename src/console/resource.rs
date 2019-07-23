use tcod::colors::*;
use specs::Entity;

#[derive(Default)]
pub struct Console {
    pub logs: Vec<Log>,
    pub y_offset: i32,
}

impl Console {
    pub fn new() -> Self {
        Self {
            logs: vec![],
            y_offset: 0,
        }
    }

    pub fn log(&mut self, log: Log) {
        self.logs.push(log.to_owned());
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.logs = vec![];
    }

    pub fn scroll(&mut self, y: i32) {
        self.y_offset += y;
        if self.y_offset < 0 {
            self.y_offset = 0;
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        self.y_offset = self.logs.len() as i32 - 1;
    }
}

#[derive(Clone)]
pub struct Log {
    pub level: LogLevel,
    pub message: String,
    pub entity: Option<Entity>,
}

impl Log {
    pub fn new<T: Into<String>>(
        level: LogLevel,
        message: T) -> Self {
        let message = message.into();
        Self {
            level,
            message,
            entity: None,
        }
    }

    #[allow(unused)]
    pub fn get_color(&self) -> Color {
        use LogLevel::*;
        match self.level {
            Game => WHITE,
            Debug => Color::new(0x60, 0x60, 0x60),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum LogLevel {
    Game,
    Debug,
}
