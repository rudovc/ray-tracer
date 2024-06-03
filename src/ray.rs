use crate::{color::Color, scene::Scene, vector::Vector3D};

pub struct Ray {
    start: Vector3D,
    direction: Vector3D,
}

impl Ray {
    pub fn new(start: &Vector3D, direction: &Vector3D) -> Self {
        Ray {
            start: start.into(),
            direction: direction.into(),
        }
    }

    pub fn trace(&self, scene: &Scene) -> Color {
        scene.background()
    }
}

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ray: {} => {}", self.start, self.direction)
    }
}
