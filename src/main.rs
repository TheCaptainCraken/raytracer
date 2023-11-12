mod image;
mod light;
mod linear_algebra;
mod material;
mod raytrace;
mod sphere;

use std::env;

use image::{Color, Image};
use light::{AmbientLight, DirectionalLight, Light, PointLight};
use linear_algebra::{vector2::Vector2, vector3::Vector3};
use material::Material;
use sphere::Sphere;

pub struct RenderSettings {
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
    let render_settings = get_settings();
    let mut canvas = Image::new(
        render_settings.canvas_width + 1,
        render_settings.canvas_height + 1,
    );
    canvas.set_all_pixels(|pos| {
        let direction = raytrace::canvas_to_viewport(&render_settings, pos.x, pos.y);
        raytrace::ray_trace(
            &render_settings,
            render_settings.camera_position,
            direction,
            render_settings.projection_plane_distance,
            f64::INFINITY,
            4,
        )
    });
    canvas.export(get_filename().as_str());
}

fn get_settings() -> RenderSettings {
    RenderSettings {
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
    }
}

fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();

    let mut file_name = String::from("untitled_render");

    if args.len() > 1 {
        file_name = args[1].clone();
    }

    file_name
}
