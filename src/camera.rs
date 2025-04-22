use crate::{
    color::Color,
    ray::Ray,
    scene::Scene,
    {vector, vector::Vector3D},
};

const ONE_HALF: f64 = 1. / 2.;

fn calculate_ndc_x(x: i32, width: u16) -> f64 {
    (x as f64 + ONE_HALF) / width as f64 * 2.0 - 1.0
}

fn calculate_ndc_y(y: i32, height: u16) -> f64 {
    1.0 - (y as f64 + 0.5) / height as f64 * 2.0
}

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
    aspect_ratio: f64,
    fov: u8,
}

impl Camera {
    pub fn new(position: Vector3D, look_at: Vector3D, width: u16, height: u16) -> Self {
        let position = if position.x() == look_at.x() && position.z() == look_at.z() {
            position.add(&Vector3D::new(0., 0., -0.0000001))
        } else {
            position
        };

        let direction = Vector3D::from(&position).to(&look_at).unit();

        let right = vector::Y.cross(&direction).unit();
        let up = right.cross(&direction).unit().invert();

        let aspect_ratio = width as f64 / height as f64;

        Camera {
            aspect_ratio,
            position,
            target: look_at,
            direction,
            width,
            height,
            right,
            up,
            fov: 60,
        }
    }

    // TODO: Revisit for arbitrary FOV and aspect ratio
    pub fn trace(&self, scene: &Scene, x: i32, y: i32) -> Color {
        let ndc_x = calculate_ndc_x(x, self.width);
        let ndc_y = calculate_ndc_y(y, self.height);

        let vx = self.right.scale(ndc_x);

        let vy = self.up.scale(ndc_y);

        let direction = self.direction.add(&vx).add(&vy);

        let ray = Ray::new(&self.position, &direction.unit());

        ray.trace(scene)
    }

    pub fn resolution(&self) -> Resolution {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{body::Sphere, color::Color, scene::Scene, utils::approx_eq, vector::Vector3D};
    use test_case::test_case;

    fn make_test_scene() -> Scene {
        let cam = Camera::new(
            Vector3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 0.0),
            600,
            600,
        );
        let sphere = Sphere::new(Vector3D::new(0.0, 0.0, 0.0), 1.0, Color::new(1, 0, 0));
        Scene::new(cam, Color::new(0, 0, 1), Box::new([Box::new(sphere)]))
    }

    #[test_case(
        Vector3D::new(0.0, 0.0, 5.0),
        Vector3D::new(0.0, 0.0,  0.0),
        600,
        600,
        Vector3D::new(0.0, 0.0, -1.0),
        Vector3D::new(1.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0)
        ; "camera axes aligned with world axes")]
    fn test_camera_new_axes(
        pos: Vector3D,
        look: Vector3D,
        w: u16,
        h: u16,
        exp_dir: Vector3D,
        exp_right: Vector3D,
        exp_up: Vector3D,
    ) {
        let cam = Camera::new(pos.clone(), look.clone(), w, h);

        assert!(approx_eq(cam.direction.x(), exp_dir.x()));
        assert!(approx_eq(cam.direction.y(), exp_dir.y()));
        assert!(approx_eq(cam.direction.z(), exp_dir.z()));

        assert!(approx_eq(cam.right.x(), exp_right.x()));
        assert!(approx_eq(cam.right.y(), exp_right.y()));
        assert!(approx_eq(cam.right.z(), exp_right.z()));

        assert!(approx_eq(cam.up.x(), exp_up.x()));
        assert!(approx_eq(cam.up.y(), exp_up.y()));
        assert!(approx_eq(cam.up.z(), exp_up.z()));
    }

    #[test_case(
        300, 300,
        (1,0,0)
        ; "we")]
    #[test_case(
        0, 0,
        (0, 0, 1)
        ; "trace misses sphere at corner pixel")]
    fn test_camera_trace_color(x: i32, y: i32, expected: (u8, u8, u8)) {
        let scene = make_test_scene();
        let color = scene.trace(x, y);

        assert_eq!(
            color.rgba(),
            Color::new(expected.0, expected.1, expected.2).rgba()
        );
    }

    #[test_case(0, 600, -0.9983333333333333     ; "ndc_x at left edge")]
    #[test_case(300, 600, 0.0016666666666667778   ; "ndc_x at center")]
    #[test_case(599, 600, 0.9983333333333333      ; "ndc_x at right edge")]
    fn test_ndc_x(x: i32, width: u16, expected: f64) {
        let val = calculate_ndc_x(x, width);
        assert!(approx_eq(val, expected));
    }

    #[test_case(0, 600, 0.9983333333333333      ; "ndc_y at top edge")]
    #[test_case(300, 600, -0.0016666666666667778  ; "ndc_y at center")]
    #[test_case(599, 600, -0.9983333333333333     ; "ndc_y at bottom edge")]
    fn test_ndc_y(y: i32, height: u16, expected: f64) {
        let val = calculate_ndc_y(y, height);
        assert!(approx_eq(val, expected));
    }
}
