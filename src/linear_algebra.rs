pub mod vector2;
pub mod vector3;

pub trait Algebra {
    fn norm(&self) -> f64;
    fn dot_product(&self, other: &Self) -> f64;
}

impl Algebra for vector2::Vector2 {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Algebra for vector3::Vector3 {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
