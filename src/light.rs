use super::linear_algebra;

#[derive(Debug, Clone, Copy)]
pub enum Light {
    Point(PointLight),
    Ambient(AmbientLight),
    Directional(DirectionalLight),
}

#[derive(Debug, Clone, Copy)]
pub struct AmbientLight {
    pub intensity: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub intensity: f64,
    pub position: linear_algebra::vector3::Vector3,
}

#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    pub intensity: f64,
    pub direction: linear_algebra::vector3::Vector3,
}
