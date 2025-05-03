use std::cmp::Ordering;

use color_eyre::eyre::Result;

use crate::{color::Color, scene::Scene, vector::Vector3D};

#[derive(Debug)]
pub struct Ray {
    pub start: Vector3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(start: &Vector3D, direction: &Vector3D) -> Self {
        Ray {
            start: start.into(),
            direction: direction.unit(),
        }
    }

    pub fn trace(&self, scene: &Scene) -> Result<Color> {
        let shortest_distance = scene
            .bodies
            .iter()
            .filter_map(|shape| {
                let distance = shape.closest_ray_distance(self);

                distance.and_then(|distance| Some((distance, shape)))
            })
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Greater));

        match shortest_distance {
            Some((distance, shape)) => {
                let way = Vector3D::from(&self.start)
                    .for_distance_in_direction(distance, &self.direction)?;

                Ok(shape.get_color_at(&way))
            }
            None => Ok(scene.background()),
        }
    }
}

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ray: {} => {}", self.start, self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utils::approx_eq, Sphere};
    use test_case::test_case;

    #[test_case(
        (0.0, 0.0, -5.0), (0.0, 0.0, 1.0), (1, 0, 0), (1, 0, 0)
        ; "ray hits sphere")]
    #[test_case(
        (0.0, 0.0, 5.0), (0.0, 0.0, 1.0), (1, 0, 0), (5, 5, 5)
        ; "ray misses all bodies")]
    fn test_ray_trace_expected_color(
        ray_start: (f64, f64, f64),
        ray_dir: (f64, f64, f64),
        sphere_color: (u8, u8, u8),
        expected_color: (u8, u8, u8),
    ) {
        let ray = Ray::new(
            &Vector3D::new(ray_start.0, ray_start.1, ray_start.2),
            &Vector3D::new(ray_dir.0, ray_dir.1, ray_dir.2),
        );

        let sphere = Sphere::new(
            Vector3D::new(0.0, 0.0, 0.0),
            1.0,
            Color::new(sphere_color.0, sphere_color.1, sphere_color.2),
        );

        let mut dummy_camera = crate::camera::Camera::new(
            &Vector3D::new(0.0, 0.0, -10.0),
            &Vector3D::new(0.0, 0.0, 0.0),
            800,
            600,
        );

        let scene = Scene::new(
            &mut dummy_camera,
            Color::new(5, 5, 5),
            Box::new([Box::new(sphere)]),
        );

        let result_color = ray.trace(&scene).unwrap();
        assert_eq!(
            result_color.rgba(),
            Color::new(expected_color.0, expected_color.1, expected_color.2).rgba()
        );
    }
    #[test_case(
    (0.0, 0.0, 0.0), (10.0, 0.0, 0.0), (1.0, 0.0, 0.0)
    ; "normalize ray direction")]
    #[test_case(
    (1.0, 2.0, 3.0), (0.0, 5.0, 0.0), (0.0, 1.0, 0.0)
    ; "normalize ray direction regardless of magnitude")]
    fn test_ray_direction_is_normalized(
        start: (f64, f64, f64),
        direction: (f64, f64, f64),
        expected_normal: (f64, f64, f64),
    ) {
        let ray = Ray::new(
            &Vector3D::new(start.0, start.1, start.2),
            &Vector3D::new(direction.0, direction.1, direction.2),
        );
        assert!(approx_eq(ray.direction.x(), expected_normal.0));
        assert!(approx_eq(ray.direction.y(), expected_normal.1));
        assert!(approx_eq(ray.direction.z(), expected_normal.2));
    }

    #[test_case(
    (1.0, 2.0, 3.0), (0.0, 1.0, 0.0)
    ; "ray starting coordinates are correct")]
    fn test_ray_start_point_correctness(start: (f64, f64, f64), direction: (f64, f64, f64)) {
        let ray = Ray::new(
            &Vector3D::new(start.0, start.1, start.2),
            &Vector3D::new(direction.0, direction.1, direction.2),
        );
        assert!(approx_eq(ray.start.x(), start.0));
        assert!(approx_eq(ray.start.y(), start.1));
        assert!(approx_eq(ray.start.z(), start.2));
    }
}
