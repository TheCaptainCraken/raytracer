use super::image;
use super::linear_algebra;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: linear_algebra::vector3::Vector3,
    pub color: image::Color,
    pub radius: f64,
    pub shininess: f64,
}
