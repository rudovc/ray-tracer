use crate::{
    color::Color,
    ray::Ray,
    scene::Scene,
    {vector, vector::Vector3D},
};

pub type Resolution = (u16, u16);

#[derive(Debug)]
pub struct Camera {
    position: Vector3D,
    target: Vector3D,
    direction: Vector3D,
    width: u16,
    height: u16,
    up: Vector3D,
    right: Vector3D,
    fov: u8,
}

impl Camera {
    pub fn new(position: Vector3D, look_at: Vector3D, width: u16, height: u16) -> Self {
        let position = if position.x() == look_at.x() && position.z() == look_at.z() {
            position.add(&Vector3D::try_new(0., 0., -0.0000001).unwrap())
        } else {
            position
        };

        let direction = Vector3D::from(&position).to(&look_at).unit();

        let right = vector::Y.cross(&direction);

        let right = right.unit().scale(width as f64 / 2.);

        let up = right
            .cross(&direction)
            .invert()
            .unit()
            .scale(height as f64 / 2.);

        Camera {
            position,
            target: look_at,
            direction,
            width,
            height,
            right,
            up,
            fov: 90,
        }
    }

    pub fn trace(&self, scene: &Scene, x: i32, y: i32) -> Color {
        let vx = self.right.scale(x as f64);
        let vy = self.right.scale(y as f64).invert();
        let r = self.direction.add(&vx).add(&vy);
        let ray = Ray::new(&self.position, &r);

        ray.trace(scene)
    }

    pub fn resolution(&self) -> Resolution {
        (self.width, self.height)
    }
}
