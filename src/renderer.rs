use sdl2::render::Canvas;

use crate::{color::Color, scene::Scene};

pub type Coordinates2D = (u16, u16);

pub struct Renderer {
    canvas_width: u16,
    canvas_height: u16,
}

impl Renderer {
    pub fn new(canvas_width: u16, canvas_height: u16) -> Self {
        Renderer {
            canvas_width,
            canvas_height,
        }
    }

    pub fn render(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        scene: &Scene,
        paint_callback: &dyn Fn(&mut Canvas<sdl2::video::Window>, Coordinates2D, Color),
    ) {
        for pixel_y in 0..self.canvas_height {
            for pixel_x in 0..self.canvas_width {
                let scene_x = (pixel_x as f32 / self.canvas_width as f32 - 0.5) * 1000.;
                let scene_y = (pixel_y as f32 / self.canvas_height as f32 - 0.5) * 1000.;

                let pixel_color = scene.trace(scene_x as i32, scene_y as i32);

                paint_callback(canvas, (pixel_x, pixel_y), pixel_color);
            }
        }
    }
}
