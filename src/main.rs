pub mod body;
pub mod camera;
pub mod color;
pub mod lazy;
pub mod ray;
pub mod renderer;
pub mod scene;
pub mod utils;
pub mod vector;
use std::{
    f64::consts::PI,
    time::{Duration, Instant},
};

use body::Sphere;
use camera::Camera;
use color::Color;
use color_eyre::Result;
use renderer::{Coordinates2D, Renderer};
use scene::Scene;
use vector::Vector3D;

use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video, VideoSubsystem};

const FULL_CIRCLE: f64 = 2. * PI;

fn get_xz_plane_rotation_from_time(
    t: Duration,
    period: u8,
    initial: &Vector3D,
    around: &Vector3D,
) -> Vector3D {
    // Drop the y term because we only care about xz plane movement
    let radius = Vector3D::from(&Vector3D::new(initial.x(), 0.0, initial.z()))
        .to(&Vector3D::new(around.x(), 0.0, around.z()))
        .length();

    // The initial position might have a phase offset, we need to account for that
    let initial_t = (period as f64 / FULL_CIRCLE) * (initial.x() / radius).asin();

    let seconds_elapsed = initial_t + t.as_millis() as f64 / 1000.;

    let x = radius * f64::sin((FULL_CIRCLE / period as f64) * seconds_elapsed) + around.x();
    let z = radius * f64::cos((FULL_CIRCLE / period as f64) * seconds_elapsed) + around.z();

    Vector3D::new(x, initial.y(), z)
}

fn initialize_window(video: VideoSubsystem, width: u16, height: u16) -> video::Window {
    video
        .window("Roko ray tracing", width.into(), height.into())
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

    let pixel_width = 600;
    let pixel_height = 600;

    let window = initialize_window(video_subsystem, pixel_width, pixel_height);

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let initial_camera_position = Vector3D::new(-10., 10., -10.);
    let target = vector::O;

    let mut camera = Camera::new(&initial_camera_position, &target, pixel_height, pixel_width);

    let mut scene = Scene::new(
        &mut camera,
        color::BLACK,
        Box::new([
            Box::new(Sphere::new(vector::O, 2., color::WHITE)),
            Box::new(Sphere::new(Vector3D::new(10., 0., 0.), 2., color::RED)),
            Box::new(Sphere::new(Vector3D::new(0., 10., 0.), 2., color::GREEN)),
            Box::new(Sphere::new(Vector3D::new(0., 0., 10.), 2., color::BLUE)),
        ]),
    );

    let renderer = Renderer::new(pixel_width, pixel_height);

    let start = Instant::now();

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
        let end = start.elapsed();

        let new_pos = get_xz_plane_rotation_from_time(end, 10, &initial_camera_position, &target);

        scene.move_camera(new_pos);
    }

    Ok(())
}
