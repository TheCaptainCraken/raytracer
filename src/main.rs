mod image;
mod light;
mod linear_algebra;
mod sphere;

use image::{Color, Image};
use light::{AmbientLight, DirectionalLight, Light, PointLight};
use linear_algebra::{
    Algebra,
    {vector2::Vector2, vector3::Vector3},
};
use sphere::Sphere;

struct State {
    canvas_width: usize,
    canvas_height: usize,
    projection_plane_distance: f64,
    objects: Vec<Sphere>,
    viewport_size: Vector2,
    camera_position: Vector3,
    camera_orientation: Vector3,
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
                color: Color::new(69, 133, 136),
                radius: 1f64,
                shininess: 300f64,
            },
            Sphere {
                center: Vector3::new(2f64, 0f64, 4f64),
                color: Color::new(215, 153, 33),
                radius: 1f64,
                shininess: 500f64,
            },
            Sphere {
                center: Vector3::new(-2f64, 0f64, 4f64),
                color: Color::new(204, 36, 29),
                radius: 1f64,
                shininess: 10f64,
            },
            Sphere {
                center: Vector3::new(0f64, -5001f64, 0f64),
                color: Color::new(142, 192, 124),
                radius: 5000f64,
                shininess: 1000f64,
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
        camera_orientation: Vector3::new(0f64, 0f64, 1f64),
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
        )
    });

    canvas.export("shadows3");
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
) -> Color {
    let intersection =
        closest_sphere_intersection(state, origin, direction, minimum_distance, maximum_distance);

    let (closest_sphere, closest_distance);

    match intersection {
        Some((sphere, distance)) => {
            closest_sphere = Some(sphere);
            closest_distance = distance;
        }
        None => {
            closest_sphere = None;
            closest_distance = f64::INFINITY;
        }
    }

    match closest_sphere {
        Some(sphere) => {
            let intersection_point = state.camera_position + direction.scaled(closest_distance);
            let intersection_point_normal = (intersection_point - sphere.center).normalized();

            let lighting_factor = compute_lighting(
                state,
                intersection_point,
                intersection_point_normal,
                direction.scaled(-1f64),
                sphere.shininess,
            );

            Color::new(
                (sphere.color.red as f64 * lighting_factor) as u8,
                (sphere.color.green as f64 * lighting_factor) as u8,
                (sphere.color.blue as f64 * lighting_factor) as u8,
            )
        }
        None => state.background_color,
    }
}

fn distance_from_intersection_with_sphere(
    origin: Vector3,
    direction: Vector3,
    sphere: &Sphere,
) -> (f64, f64) {
    let distance_from_center = origin - sphere.center;

    let a = direction.dot(&direction);
    let b = 2f64 * distance_from_center.dot(&direction);
    let c = distance_from_center.dot(&distance_from_center) - (sphere.radius * sphere.radius);

    let discriminant = b * b - 4f64 * a * c;

    if discriminant < 0f64 {
        (f64::INFINITY, f64::INFINITY)
    } else {
        let t1 = (-b + f64::sqrt(discriminant)) / (2f64 * a);
        let t2 = (-b - f64::sqrt(discriminant)) / (2f64 * a);

        (t1, t2)
    }
}

fn closest_sphere_intersection(
    state: &State,
    origin: Vector3,
    direction: Vector3,
    minimum_distance: f64,
    maximum_distance: f64,
) -> Option<(&Sphere, f64)> {
    let mut closest_distance = f64::INFINITY;
    let mut closest_sphere = None;

    for sphere in &state.objects {
        let (distance1, distance2) =
            distance_from_intersection_with_sphere(origin, direction, sphere);
        if distance1 < closest_distance
            && distance1 >= minimum_distance
            && distance1 <= maximum_distance
        {
            closest_distance = distance1;
            closest_sphere = Some(sphere);
        }
        if distance2 < closest_distance
            && distance2 >= minimum_distance
            && distance2 <= maximum_distance
        {
            closest_distance = distance2;
            closest_sphere = Some(sphere);
        }
    }

    match closest_sphere {
        Some(sphere) => Some((sphere, closest_distance)),
        None => None,
    }
}

fn compute_lighting(
    state: &State,
    point: Vector3,
    normal: Vector3,
    direction: Vector3,
    shininess: f64,
) -> f64 {
    let mut lighting_factor: f64 = 0f64;

    for light in &state.lights {
        match light {
            Light::Ambient(ambient_light) => lighting_factor += ambient_light.intensity,
            Light::Point(point_light) => {
                let light_direction = point_light.position - point;

                let intersection =
                    closest_sphere_intersection(state, point, light_direction, 0.00001f64, 1f64);

                match intersection {
                    Some(_) => continue,
                    None => (),
                };

                let dot_product = normal.dot(&light_direction);
                if dot_product > 0f64 {
                    lighting_factor += point_light.intensity * dot_product
                        / (normal.norm() * light_direction.norm());
                }

                let perfect_reflection_direction =
                    normal.scaled(2f64 * normal.dot(&light_direction)) - light_direction;

                let useful_dot_prod = perfect_reflection_direction.dot(&direction);

                if useful_dot_prod > 0f64 {
                    lighting_factor += point_light.intensity
                        * (useful_dot_prod
                            / (direction.norm() * perfect_reflection_direction.norm()))
                        .powf(shininess)
                }
            }
            Light::Directional(directional_light) => {
                let intersection = closest_sphere_intersection(
                    state,
                    point,
                    directional_light.direction,
                    0.00001f64,
                    f64::INFINITY,
                );

                match intersection {
                    Some(_) => continue,
                    None => (),
                };

                let dot_product = normal.dot(&directional_light.direction);
                if dot_product > 0f64 {
                    lighting_factor += directional_light.intensity * dot_product
                        / (normal.norm() * directional_light.direction.norm());
                }

                let perfect_reflection_direction = normal
                    .scaled(2f64 * normal.dot(&directional_light.direction))
                    - directional_light.direction;

                let useful_dot_prod = perfect_reflection_direction.dot(&direction);

                if useful_dot_prod > 0f64 {
                    lighting_factor += directional_light.intensity
                        * (useful_dot_prod
                            / (direction.norm() * perfect_reflection_direction.norm()))
                        .powf(shininess)
                }
            }
        };
    }

    lighting_factor
}
