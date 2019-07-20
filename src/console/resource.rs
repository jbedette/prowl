use tcod::colors::*;

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

    pub fn log(&mut self, message: Log) {
        // TODO is to_owned best here?
        self.logs.push(message.to_owned());
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.logs = vec![];
    }
}

#[derive(Clone)]
pub struct Log {
    pub level: LogLevel,
    pub message: String,
}

impl Log {
    pub fn new<T: Into<String>>(level: LogLevel, message: T) -> Self {
        let message = message.into();
        Self { level, message }
    }

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