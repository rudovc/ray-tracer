pub mod camera;
pub mod color;
pub mod ray;
pub mod renderer;
pub mod scene;
pub mod vector;
use camera::Camera;
use color::Color;
use color_eyre::Result;
use renderer::{Coordinates2D, Renderer};
use scene::Scene;
use std::time::Duration;
use vector::Vector3D;

use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video, VideoSubsystem};

fn initialize_window(video: VideoSubsystem) -> video::Window {
    video
        .window("SDL2 tutorial", 1024, 768)
        .position_centered()
        .build()
        .unwrap()
}

fn get_pixel_color_for_coordinates_chessboard(x: u16, y: u16) -> color::Color {
    let checker_size = 64;
    let x_odd = x % (2 * checker_size) < checker_size;
    let y_odd = y % (2 * checker_size) < checker_size;

    if x_odd != y_odd {
        color::WHITE
    } else {
        color::BLACK
    }
}

fn paint_pixel(canvas: &mut Canvas<sdl2::video::Window>, (x, y): Coordinates2D, color: Color) {
    canvas.set_draw_color(color);
    canvas
        .draw_point((x as i32, y as i32))
        .unwrap_or_else(|_| panic!("Could not draw color {color:?} to point {x}, {y}."));
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = initialize_window(video_subsystem);

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let camera = Camera::new(
        Vector3D::try_new(-4., 1., -5.).unwrap(),
        vector::O,
        1024,
        768,
    );

    let scene = Scene::new(camera, color::BLUE);
    let renderer = Renderer::new(1024, 768);

    'running: loop {
        renderer.render(&mut canvas, &scene, &paint_pixel);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
