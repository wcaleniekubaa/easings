use std::str::FromStr;

use clap::Parser;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{FPoint, Rect};

#[derive(Debug, Clone, Parser)]
pub struct Arg {
    /// Easing type, see [easing::Type]
    #[arg(short, long, default_value = "Linear")]
    pub easing: String
}

fn main() {
    let args = Arg::parse();

    let easing = easings::Type::from_str(&args.easing).unwrap();

    println!("Press F2 to render graph to image [graph.png]");

    let sdl = sdl2::init().unwrap();

    let video = sdl.video().unwrap();

    let window = video
        .window(&format!("Easing: {}", easing.name()), 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let points = (0..1000)
        .map(|x| x as f64 / 1000.0)
        .map(|x| (x, 1.0 - easing.get(x)))
        .map(|(x, y)| (x * (800.0 * 0.8) / 3.0, y * (600.0 * 0.8) / 3.0))
        .map(|(x, y)| (x + (800.0 * 0.2) / 6.0, y + (600.0 * 0.2) / 6.0))
        .map(|(x, y)| FPoint::new(x as f32, y as f32))
        .collect::<Vec<_>>();

    canvas.set_draw_color((20, 20, 20));
    canvas.clear();

    canvas.set_scale(3.0, 3.0).unwrap();

    canvas.set_draw_color(Color::BLUE);
    canvas.draw_flines(points.as_slice()).unwrap();

    canvas.present();

    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::F2),
                    ..
                } => {
                    let pixels = canvas
                        .read_pixels(Some(Rect::new(0, 0, 800, 600)), PixelFormatEnum::RGBA32)
                        .unwrap();

                    let buffer =
                        image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(800, 600, pixels)
                            .unwrap();

                    buffer.save("graph.png").unwrap();
                }
                _ => {}
            }
        }
    }
}
