use color_eyre::eyre::Result;
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
    ) -> Result<()> {
        for pixel_y in 0..self.canvas_height {
            for pixel_x in 0..self.canvas_width {
                let pixel_color = scene.trace(pixel_x as i32, pixel_y as i32)?;

                paint_callback(canvas, (pixel_x, pixel_y), pixel_color);
            }
        }

        Ok(())
    }
}
