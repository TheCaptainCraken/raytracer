mod image;
mod linear_algebra;
mod sphere;

use image::{Image, Pixel};
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
    background_color: Pixel,
}

fn main() {
    let state = State {
        canvas_width: 1920,
        canvas_height: 1080,
        projection_plane_distance: 1f64,
        objects: vec![
            Sphere {
                center: Vector3::new(0f64, -1f64, 3f64),
                color: Pixel::new(250, 230, 30),
                radius: 1f64,
            },
            Sphere {
                center: Vector3::new(2f64, 0f64, 4f64),
                color: Pixel::new(250, 30, 230),
                radius: 1f64,
            },
            Sphere {
                center: Vector3::new(-2f64, 0f64, 4f64),
                color: Pixel::new(30, 250, 230),
                radius: 1f64,
            },
        ],
        viewport_size: Vector2::new(1f64, 1f64),
        camera_position: Vector3::new(0f64, 0f64, 0f64),
        camera_orientation: Vector3::new(0f64, 0f64, 1f64),
        background_color: Pixel::new(30, 30, 30),
    };

    let mut canvas = Image::new(state.canvas_width, state.canvas_height);

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

    canvas.export("first_render3");
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
) -> Pixel {
    let mut closest_distance = f64::INFINITY;
    let mut closest_sphere = None;

    for sphere in &state.objects {
        let (distance1, distance2) =
            distance_from_intersection_with_sphere(origin, direction, sphere);
        if distance1 < closest_distance
            && distance1 > minimum_distance
            && distance1 < maximum_distance
        {
            closest_distance = distance1;
            closest_sphere = Some(sphere);
        } else if distance2 < closest_distance
            && distance2 > minimum_distance
            && distance2 < maximum_distance
        {
            closest_distance = distance2;
            closest_sphere = Some(sphere);
        }
    }

    match closest_sphere {
        Some(sphere) => sphere.color,
        None => state.background_color,
    }
}

fn distance_from_intersection_with_sphere(
    origin: Vector3,
    direction: Vector3,
    sphere: &Sphere,
) -> (f64, f64) {
    let distance_from_center = origin - sphere.center;

    let a = direction.dot_product(&direction);
    let b = 2f64 * distance_from_center.dot_product(&direction);
    let c = distance_from_center.dot_product(&distance_from_center) - sphere.radius * sphere.radius;

    let discriminant = b * b - 4f64 * a * c;

    if discriminant < 0f64 {
        (f64::INFINITY, f64::INFINITY)
    } else {
        let t1 = (-b + f64::sqrt(discriminant)) / (2f64 * a);
        let t2 = (-b - f64::sqrt(discriminant)) / (2f64 * a);

        (t1, t2)
    }
}
