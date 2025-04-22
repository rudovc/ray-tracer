use derivative::Derivative;

use crate::{body::Renderable, camera::Camera, color::Color};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Scene {
    camera: Camera,
    background: Color,
    #[derivative(Debug = "ignore")]
    pub bodies: Vec<Box<dyn Renderable>>,
}

impl Scene {
    pub fn new(camera: Camera, background: Color, bodies: Box<[Box<dyn Renderable>]>) -> Self {
        Scene {
            camera,
            background,
            bodies: bodies.into(),
        }
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn trace(&self, x: i32, y: i32) -> Color {
        self.camera.trace(self, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector3D;
    use test_case::test_case;

    #[test_case((2, 3, 4) ; "Scene returns correct background color")]
    fn test_scene_background(expected_color: (u8, u8, u8)) {
        let dummy_camera = crate::camera::Camera::new(
            Vector3D::new(0.0, 0.0, -10.0),
            Vector3D::new(0.0, 0.0, 0.0),
            800,
            600,
        );

        let scene = Scene::new(
            dummy_camera,
            Color::new(expected_color.0, expected_color.1, expected_color.2),
            vec![].into_boxed_slice(),
        );

        assert_eq!(
            scene.background().rgba(),
            Color::new(expected_color.0, expected_color.1, expected_color.2).rgba()
        );
    }
}
