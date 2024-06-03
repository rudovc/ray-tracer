pub mod color;
use color_eyre::Result;
use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, video, VideoSubsystem};

fn initialize_window(video: VideoSubsystem) -> video::Window {
    video
        .window("SDL2 tutorial", 1024, 768)
        .position_centered()
        .build()
        .unwrap()
}

fn get_chessboard_color_for_pixel(x: u16, y: u16) -> color::Color {
    let checker_size = 64;
    let x_odd = x % (2 * checker_size) < checker_size;
    let y_odd = y % (2 * checker_size) < checker_size;

    if x_odd != y_odd {
        color::WHITE
    } else {
        color::BLACK
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = initialize_window(video_subsystem);

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for x in 0..1024 {
            for y in 0..768 {
                let pixel_color = get_chessboard_color_for_pixel(x, y);
                canvas.set_draw_color(&pixel_color);

                canvas.draw_point((x as i32, y as i32)).unwrap_or_else(|_| {
                    panic!("Could not draw color {pixel_color:?} to point {x}, {y}.")
                });
            }
        }

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
