// normalizes values between 0 and 1
// TODO make generic? if needed again?
/*
pub fn normalize_vec(vec: Vec<f64>, min: f64, max: f64) -> Vec<f64> {
    vec.into_iter().map(|value|
            (value - min) / (max - min)
        ).collect()
}
*/
use std::ops::{
    Sub,
    Div,
};

pub fn normalize_vec<T>(vec: Vec<T>, min: T, max: T) -> Vec<T>
where T: Sub<Output=T> + Div<Output=T> + Copy
{
    vec.into_iter().map(|value|
            (value - min) / (max - min)
        ).collect()
}
