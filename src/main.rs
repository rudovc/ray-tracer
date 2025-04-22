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

fn initialize_window(video: VideoSubsystem, width: u16, height: u16) -> video::Window {
    video
        .window("SDL2 tutorial", width.into(), height.into())
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

    let pixel_width = 1000;
    let pixel_height = 1000;

    let window = initialize_window(video_subsystem, pixel_width, pixel_height);

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let camera = Camera::new(
        Vector3D::new(-10., 10., -10.),
        vector::O,
        pixel_height,
        pixel_width,
    );

    let scene = Scene::new(
        camera,
        color::BLACK,
        Box::new([
            Box::new(Sphere::new(vector::O, 2., color::WHITE)),
            Box::new(Sphere::new(Vector3D::new(10., 0., 0.), 2., color::RED)),
            Box::new(Sphere::new(Vector3D::new(0., 10., 0.), 2., color::GREEN)),
            Box::new(Sphere::new(Vector3D::new(0., 0., 10.), 2., color::BLUE)),
        ]),
    );

    let renderer = Renderer::new(pixel_width, pixel_height);

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
