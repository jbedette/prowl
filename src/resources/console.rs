use tcod::{
    colors::*,
};

#[derive(Default)]
pub struct Console {
    pub logs: Vec<Log>,
}

impl Console {
    pub fn new() -> Self {
        Self { logs: vec![] }
    }

    pub fn log(&mut self, message: Log) {
        // TODO is to_owned best here?
        self.logs.push(message.to_owned());
    }
}

#[derive(Clone)]
pub struct Log {
    pub level: LogLevel,
    pub message: String,
    // entity: Entity, // maybe better than grabbing name all the time...
}

impl Log {
    pub fn new(level: LogLevel, message: &str) -> Self {
        let message = String::from(message);
        Self {
            level,
            message
        }
    }

    pub fn get_color(&self) -> Color {
        use LogLevel::*;
        match self.level {
            Game => WHITE,
            Debug => YELLOW,
        }
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum LogLevel {
    Game,
    Debug,
}
