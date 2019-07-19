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
/*
trait Num {}
impl Num for usize {}
impl Num for u8 {}
impl Num for u16 {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for i8 {}
impl Num for i16 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for f32 {}
impl Num for f64 {}

#[derive(Default, Copy, Clone)]
pub struct Vector2<N: Num> {
    pub x: N,
    pub y: N,
}

impl<N: Num> Vector2<N> {
    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
    }

    pub fn to_tuple(&self) -> (N, N) {
        let vector2 = self.to_owned();
        (vector2.x, vector2.y)
    }
}
*/
