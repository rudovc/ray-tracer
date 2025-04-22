pub const THRESHOLD: f64 = f64::EPSILON * 3.;

use std::cmp::Ordering;

use crate::{color::Color, ray::Ray, vector::Vector3D};

#[derive(Debug)]
pub struct Body {
    color: Color,
}

impl Body {
    pub fn new(color: Color) -> Self {
        Body { color }
    }
}

pub trait Colored {
    fn color(&self) -> Color;
}

impl Colored for Body {
    fn color(&self) -> Color {
        self.color
    }
}

pub trait Volume {
    fn closest_ray_point(&self, ray: &Ray) -> Option<f64>;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
}

pub trait Renderable: Volume + Colored {}

#[derive(Debug)]
pub struct Sphere {
    body: Body,
    center: Vector3D,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f64, color: Color) -> Self {
        Sphere {
            body: Body { color },
            radius,
            center,
        }
    }
}

impl Colored for Sphere {
    fn color(&self) -> Color {
        self.body.color()
    }
}

impl Volume for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        // For this system, the sphere's center is the origin
        let ray_start_coordinate = Vector3D::from(&self.center).to(&ray.start);

        let b = 2. * ray_start_coordinate.dot(&ray.direction);
        let c = ray_start_coordinate.squid() - self.radius * self.radius;

        let discriminant = b * b - 4. * c;

        if discriminant < 0. {
            vec![]
        } else if discriminant == 0. {
            vec![-b / 2.]
        } else {
            let root = discriminant.sqrt();
            vec![(-b - root) / 2., (-b + root) / 2.]
        }
    }

    fn closest_ray_point(&self, ray: &Ray) -> Option<f64> {
        let distances = self
            .intersect(ray)
            .into_iter()
            .filter(|distance| *distance > THRESHOLD);

        distances.min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Greater))
    }
}

impl Renderable for Sphere {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::ray::Ray;
    use crate::utils::approx_eq;
    use test_case::test_case;

    #[test_case((1, 2, 3) ; "body stores and returns its color correctly")]
    fn test_body_color(initial: (u8, u8, u8)) {
        let c = Color::new(initial.0, initial.1, initial.2);
        let body = Body::new(c);

        assert_eq!(body.color().rgba(), c.rgba());
    }

    #[test_case((1.0, 2.0, 3.0), 5.0, (4, 5, 6) ; "sphere preserves center, radius, and color")]
    fn test_sphere_fields(center: (f64, f64, f64), radius: f64, color: (u8, u8, u8)) {
        let cen = Vector3D::new(center.0, center.1, center.2);
        let col = Color::new(color.0, color.1, color.2);
        let sphere = Sphere::new(cen, radius, col);
        assert!(approx_eq(sphere.center.x(), center.0));
        assert!(approx_eq(sphere.center.y(), center.1));
        assert!(approx_eq(sphere.center.z(), center.2));
        assert!(approx_eq(sphere.radius, radius));

        assert_eq!(sphere.color().rgba(), col.rgba())
    }

    #[test_case(
        (0.0, 0.0, 5.0), (0.0, 1.0, 0.0), vec![], None
        ; "ray misses sphere")]
    #[test_case(
        (1.0, -5.0, 0.0), (0.0, 1.0, 0.0), vec![5.0], Some(5.0)
        ; "ray tangent to sphere returns correct t = -b/2")]
    #[test_case(
        (0.0, 0.0, -5.0), (0.0, 0.0, 1.0), vec![4.0, 6.0], Some(4.0)
        ; "ray pierces sphere twice")]
    #[test_case(
        (0.0, 0.0, 0.0), (1.0, 0.0, 0.0), vec![-1.0, 1.0], Some(1.0)
        ; "ray origin inside sphere")]
    fn test_sphere_intersection(
        start: (f64, f64, f64),
        direction: (f64, f64, f64),
        expected_ts: Vec<f64>,
        expected_closest: Option<f64>,
    ) {
        let sphere = Sphere::new(Vector3D::new(0.0, 0.0, 0.0), 1.0, Color::new(0, 0, 0));
        let ray = Ray {
            start: Vector3D::new(start.0, start.1, start.2),
            direction: Vector3D::new(direction.0, direction.1, direction.2),
        };
        let mut intersections = sphere.intersect(&ray);
        assert!(intersections.iter().all(|t| t.is_finite()));
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(intersections, expected_ts);
        let closest = sphere.closest_ray_point(&ray);
        assert_eq!(closest, expected_closest);
    }
}
