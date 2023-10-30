use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, other: Self) -> Self::Output {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, other: Self) -> Self::Output {
        Vector2::new(self.x * other.x, self.y * other.y)
    }
}

impl Div for Vector2 {
    type Output = Vector2;
    fn div(self, other: Self) -> Self::Output {
        Vector2::new(self.x / other.x, self.y / other.y)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.x, self.y))
    }
}
