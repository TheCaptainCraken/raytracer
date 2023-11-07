mod image;
mod light;
mod linear_algebra;
mod material;
mod sphere;

use image::{Color, Image};
use light::{compute_lighting, AmbientLight, DirectionalLight, Light, PointLight};
use linear_algebra::{
    Algebra,
    {vector2::Vector2, vector3::Vector3},
};
use material::Material;
use sphere::Sphere;

pub struct State {
    canvas_width: usize,
    canvas_height: usize,
    projection_plane_distance: f64,
    objects: Vec<Sphere>,
    viewport_size: Vector2,
    camera_position: Vector3,
    background_color: Color,
    lights: Vec<light::Light>,
}

fn main() {
    let state = State {
        canvas_width: 1920,
        canvas_height: 1920,
        projection_plane_distance: 1f64,
        objects: vec![
            Sphere {
                center: Vector3::new(0f64, -1f64, 3f64),
                radius: 1f64,
                material: Material::new(Color::new(255, 255, 0), None, None),
            },
            Sphere {
                center: Vector3::new(2f64, 0f64, 4f64),
                material: Material::new(Color::new(0, 255, 255), Some(500f64), Some(0.3f64)),
                radius: 1f64,
            },
            Sphere {
                center: Vector3::new(-2f64, 0f64, 4f64),
                material: Material::new(Color::new(255, 0, 255), Some(10f64), Some(0.4f64)),
                radius: 1f64,
            },
            Sphere {
                center: Vector3::new(0f64, -5001f64, 0f64),
                material: Material::new(Color::new(255, 0, 0), None, Some(0.5f64)),
                radius: 5000f64,
            },
            Sphere {
                center: Vector3::new(0f64, 2f64, 3f64),
                material: Material::new(Color::new(230, 230, 230), None, Some(0.8f64)),
                radius: 1.5f64,
            },
        ],
        lights: vec![
            Light::Ambient(AmbientLight { intensity: 0.2 }),
            Light::Point(PointLight {
                intensity: 0.6,
                position: Vector3::new(2f64, 1f64, 0f64),
            }),
            Light::Directional(DirectionalLight {
                intensity: 0.2,
                direction: Vector3::new(1f64, 4f64, 4f64),
            }),
        ],
        viewport_size: Vector2::new(1f64, 1f64),
        camera_position: Vector3::new(0f64, 1f64, -5f64),
        background_color: Color::new(60, 56, 54),
    };

    let mut canvas = Image::new(state.canvas_width + 1, state.canvas_height + 1);

    canvas.set_all_pixels(|pos| {
        let direction = canvas_to_viewport(&state, pos.x, pos.y);
        ray_trace(
            &state,
            state.camera_position,
            direction,
            state.projection_plane_distance,
            f64::INFINITY,
            4,
        )
    });

    canvas.export("reflections4");
}

fn canvas_to_viewport(state: &State, x: i64, y: i64) -> Vector3 {
    Vector3::new(
        (x as f64) * state.viewport_size.x / (state.canvas_width as f64),
        (y as f64) * state.viewport_size.y / (state.canvas_height as f64),
        state.projection_plane_distance,
    )
}

fn ray_trace(
    state: &State,
    origin: Vector3,
    direction: Vector3,
    minimum_distance: f64,
    maximum_distance: f64,
    depth: usize,
) -> Color {
    let intersection = Sphere::closest_sphere_intersection(
        &state.objects,
        origin,
        direction,
        minimum_distance,
        maximum_distance,
    );

    match intersection {
        Some((sphere, closest_distance)) => {
            let intersection_point = state.camera_position + direction.scaled(closest_distance);
            let intersection_point_normal = (intersection_point - sphere.center).normalized();

            let lighting_factor = compute_lighting(
                state,
                intersection_point,
                intersection_point_normal,
                direction.inverse(),
                sphere.material.shininess,
            );

            let local_color = Color::new(
                (sphere.material.surface_color.red as f64 * lighting_factor) as u8,
                (sphere.material.surface_color.green as f64 * lighting_factor) as u8,
                (sphere.material.surface_color.blue as f64 * lighting_factor) as u8,
            );

            if depth == 0 {
                local_color
            } else {
                let reflected_ray = reflect_ray(direction.inverse(), intersection_point_normal);
                let reflected_color = ray_trace(
                    state,
                    intersection_point,
                    reflected_ray,
                    0.001,
                    maximum_distance,
                    depth - 1,
                );

                match sphere.material.reflectivness {
                    Some(r) => Color::new(
                        ((local_color.red as f64) * (1f64 - r) + (reflected_color.red as f64) * r)
                            as u8,
                        ((local_color.green as f64) * (1f64 - r)
                            + (reflected_color.green as f64) * r) as u8,
                        ((local_color.blue as f64) * (1f64 - r) + (reflected_color.blue as f64) * r)
                            as u8,
                    ),
                    None => local_color,
                }
            }
        }
        None => state.background_color,
    }
}

fn reflect_ray(ray: Vector3, normal: Vector3) -> Vector3 {
    normal.scaled(2f64 * normal.dot(&ray)) - ray
}
