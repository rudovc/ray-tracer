const THRESHOLD: f64 = f64::EPSILON * 3.;

use std::cmp::Ordering;

use crate::{color::Color, ray::Ray, vector::Vector3D};

#[derive(Debug)]
pub struct Body {
    color: Color,
}

impl Body {
    pub fn new(&self, color: Color) -> Self {
        Body { color }
    }
}

pub trait Volume {
    fn closest_ray_point(&self, ray: &Ray) -> Option<f64>;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
}

#[derive(Debug)]
pub struct Sphere {
    body: Body,
    center: Vector3D,
    radius: f64,
}

impl Sphere {
    pub fn new(&self, center: Vector3D, radius: f64, color: Color) -> Self {
        Sphere {
            body: Body { color },
            radius,
            center,
        }
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
            vec![discriminant]
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
