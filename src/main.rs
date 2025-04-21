pub mod body;
pub mod camera;
pub mod color;
pub mod lazy;
pub mod ray;
pub mod renderer;
pub mod scene;
pub mod utils;
pub mod vector;
use body::Sphere;
use camera::Camera;
use color::Color;
use color_eyre::Result;
use renderer::{Coordinates2D, Renderer};
use scene::Scene;
use vector::Vector3D;

use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video, VideoSubsystem};

fn initialize_window(video: VideoSubsystem) -> video::Window {
    video
        .window("SDL2 tutorial", 1024, 768)
        .position_centered()
        .build()
        .unwrap()
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

    let camera = Camera::new(Vector3D::new(-4., 5., -5.), vector::O, 1024, 768);

    let scene = Scene::new(
        camera,
        color::BLUE,
        Box::new([Box::new(Sphere::new(
            Vector3D::new(0., 0., 0.),
            1.,
            color::RED,
        ))]),
    );

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
    }

    Ok(())
}
