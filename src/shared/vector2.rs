/// Any value with an x, y
use std::{
    fmt,
    ops,
};

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn north() -> Self {
        Self { x: 0, y: -1 }
    }

    pub fn south() -> Self {
        Self { x: 0, y: 1 }
    }

    pub fn east() -> Self {
        Self { x: 1, y: 0 }
    }

    pub fn west() -> Self {
        Self { x: -1, y: 0 }
    }

    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}

impl ops::Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
