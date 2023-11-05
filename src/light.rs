use crate::{
    linear_algebra::{vector3::Vector3, Algebra},
    sphere::closest_sphere_intersection,
    State,
};

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

pub fn compute_lighting(
    state: &State,
    point: Vector3,
    normal: Vector3,
    v: Vector3,
    shininess: f64,
) -> f64 {
    let mut lighting_factor: f64 = 0f64;

    for light in &state.lights {
        match light {
            Light::Ambient(ambient_light) => lighting_factor += ambient_light.intensity,
            Light::Point(point_light) => {
                let light_direction = point_light.position - point;
                if !is_in_shadow(state, point, light_direction, 0.000001f64, 1f64) {
                    lighting_factor +=
                        compute_illumination(point_light.intensity, light_direction, normal)
                            + compute_specular_reflections(
                                point_light.intensity,
                                light_direction,
                                normal,
                                shininess,
                                v,
                            );
                }
            }
            Light::Directional(directional_light) => {
                if !is_in_shadow(state, point, directional_light.direction, 0.000001f64, 1f64) {
                    lighting_factor += compute_illumination(
                        directional_light.intensity,
                        directional_light.direction,
                        normal,
                    ) + compute_specular_reflections(
                        directional_light.intensity,
                        directional_light.direction,
                        normal,
                        shininess,
                        v,
                    );
                }
            }
        };
    }

    lighting_factor
}

fn is_in_shadow(
    state: &State,
    point: Vector3,
    direction: Vector3,
    minimum_distance: f64,
    maximum_distance: f64,
) -> bool {
    closest_sphere_intersection(state, point, direction, minimum_distance, maximum_distance)
        .is_some()
}

fn compute_illumination(intensity: f64, direction: Vector3, normal: Vector3) -> f64 {
    let dot_product = normal.dot(&direction);
    if dot_product > 0f64 {
        intensity * dot_product / (normal.norm() * direction.norm())
    } else {
        0f64
    }
}

fn compute_specular_reflections(
    intensity: f64,
    direction: Vector3,
    normal: Vector3,
    shininess: f64,
    v: Vector3,
) -> f64 {
    let perfect_reflection_direction = normal.scaled(2f64 * normal.dot(&direction)) - direction;

    let useful_dot_prod = perfect_reflection_direction.dot(&v);

    if useful_dot_prod > 0f64 {
        intensity
            * (useful_dot_prod / (v.norm() * perfect_reflection_direction.norm())).powf(shininess)
    } else {
        0f64
    }
}
