use crate::{camera::Camera, color::Color};

#[derive(Debug)]
pub struct Scene {
    camera: Camera,
    background: Color,
}

impl Scene {
    pub fn new(camera: Camera, background: Color) -> Self {
        Scene { camera, background }
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn trace(&self, x: u16, y: u16) -> Color {
        self.camera.trace(self, x, y)
    }
}
