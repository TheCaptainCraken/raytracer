use crate::{
    image::Color,
    light::compute_lighting,
    linear_algebra::{vector3::Vector3, Algebra},
    sphere::Sphere,
    RenderSettings,
};

pub fn canvas_to_viewport(state: &RenderSettings, x: i64, y: i64) -> Vector3 {
    Vector3::new(
        (x as f64) * state.viewport_size.x / (state.canvas_width as f64),
        (y as f64) * state.viewport_size.y / (state.canvas_height as f64),
        state.projection_plane_distance,
    )
}

pub fn ray_trace(
    render_settings: &RenderSettings,
    origin: Vector3,
    direction: Vector3,
    minimum_distance: f64,
    maximum_distance: f64,
    depth: usize,
) -> Color {
    let intersection = Sphere::closest_sphere_intersection(
        &render_settings.objects,
        origin,
        direction,
        minimum_distance,
        maximum_distance,
    );

    match intersection {
        Some((sphere, closest_distance)) => {
            let intersection_point =
                render_settings.camera_position + direction.scaled(closest_distance);
            let intersection_point_normal = (intersection_point - sphere.center).normalized();

            let lighting_factor = compute_lighting(
                render_settings,
                intersection_point,
                intersection_point_normal,
                direction.inverse(),
                sphere.material.shininess,
            );

            let local_color = Color::from_vec3(
                sphere
                    .material
                    .surface_color
                    .to_vec3()
                    .scaled(lighting_factor),
            );

            if depth == 0 {
                local_color
            } else {
                let reflected_ray = reflect_ray(direction.inverse(), intersection_point_normal);
                let reflected_color = ray_trace(
                    render_settings,
                    intersection_point,
                    reflected_ray,
                    0.001,
                    maximum_distance,
                    depth - 1,
                );

                match sphere.material.reflectivness {
                    Some(r) => Color::from_vec3(
                        local_color.to_vec3().scaled(1f64 - r)
                            + reflected_color.to_vec3().scaled(r),
                    ),
                    None => local_color,
                }
            }
        }
        None => render_settings.background_color,
    }
}

fn reflect_ray(ray: Vector3, normal: Vector3) -> Vector3 {
    normal.scaled(2f64 * normal.dot(&ray)) - ray
}
