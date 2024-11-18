use derivative::Derivative;

use crate::{camera::Camera, color::Color, shape::Volume};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Scene {
    camera: Camera,
    background: Color,
    #[derivative(Debug = "ignore")]
    shapes: Vec<Box<dyn Volume>>,
}

impl Scene {
    pub fn new(camera: Camera, background: Color, shapes: Box<[Box<dyn Volume>]>) -> Self {
        Scene {
            camera,
            background,
            shapes: shapes.into(),
        }
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn trace(&self, x: i32, y: i32) -> Color {
        self.camera.trace(self, x, y)
    }
}
