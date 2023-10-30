use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Self) -> Self::Output {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Self) -> Self::Output {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div for Vector3 {
    type Output = Vector3;
    fn div(self, other: Self) -> Self::Output {
        Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Self) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}, {}]", self.x, self.y, self.z))
    }
}
