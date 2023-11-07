use crate::image::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub surface_color: Color,
    pub shininess: Option<f64>,
    pub reflectivness: Option<f64>,
}

impl Material {
    pub fn new(surface_color: Color, shininess: Option<f64>, reflectivness: Option<f64>) -> Self {
        Material {
            surface_color,
            shininess,
            reflectivness,
        }
    }
}
