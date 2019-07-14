/// Any value with an x, y

#[derive(Default, Copy, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
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
