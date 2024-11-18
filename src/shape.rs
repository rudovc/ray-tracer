const THRESHOLD: f64 = f64::EPSILON * 3.;

use crate::{color::Color, ray::Ray, vector::Vector3D};

pub enum Intersection {
    NONE,
    TANGENT(f64),
    PIERCE([f64; 2]),
}

pub struct Body {
    color: Color,
}

impl Body {
    pub fn new(&self, color: Color) -> Self {
        Body { color }
    }
}

pub trait Volume {
    fn intersect(&self, ray: Ray) -> Intersection;
}

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
    fn intersect(&self, ray: Ray) -> Intersection {
        // For this system, the sphere's center is the origin
        let ray_start_coordinate = Vector3D::from(&self.center).to(&ray.start);

        let b = 2. * ray_start_coordinate.dot(&ray.direction);
        let c = ray_start_coordinate.squid() - self.radius * self.radius;

        let discriminant = b * b - 4. * c;

        if discriminant < 0. {
            Intersection::NONE
        } else if discriminant.abs() <= THRESHOLD {
            Intersection::TANGENT(discriminant)
        } else {
            let root = discriminant.sqrt();
            Intersection::PIERCE([(-b - root) / 2., (-b + root) / 2.])
        }
    }
}
