use super::linear_algebra;
use crate::linear_algebra::vector3::Vector3;
use crate::linear_algebra::Algebra;
use crate::material::Material;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: linear_algebra::vector3::Vector3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn closest_sphere_intersection(
        spheres: &Vec<Sphere>,
        origin: Vector3,
        direction: Vector3,
        minimum_distance: f64,
        maximum_distance: f64,
    ) -> Option<(&Sphere, f64)> {
        let mut closest_distance = f64::INFINITY;
        let mut closest_sphere = None;

        for sphere in spheres {
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
