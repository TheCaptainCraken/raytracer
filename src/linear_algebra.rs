pub mod vector2;
pub mod vector3;

pub trait Algebra {
    fn norm(&self) -> f64;
    fn dot(&self, other: &Self) -> f64;
    fn scaled(&self, factor: f64) -> Self;
    fn normalized(&self) -> Self;
}

impl Algebra for vector2::Vector2 {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn scaled(&self, factor: f64) -> Self {
        vector2::Vector2::new(self.x * factor, self.y * factor)
    }

    fn normalized(&self) -> Self {
        let norm = self.norm();
        self.scaled(1f64 / norm)
    }
}

impl Algebra for vector3::Vector3 {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn scaled(&self, factor: f64) -> Self {
        vector3::Vector3::new(self.x * factor, self.y * factor, self.z * factor)
    }

    fn normalized(&self) -> Self {
        let norm = self.norm();
        self.scaled(1f64 / norm)
    }
}
