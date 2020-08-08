use color::{Deg, Hsv, ToRgb};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    // init window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width = 640;
    let height = 480;
    let window = video_subsystem
        .window("SDL", width, height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // draw background color
    let mut deg = Deg { 0: 0. };
    let rgb = Hsv::new(deg, 1., 1.).to_rgb();
    canvas.set_draw_color(Color::RGB(rgb.r, rgb.g, rgb.b));
    canvas.clear();

    // render
    canvas.present();

    // render loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // rotate background color
        deg = Deg { 0: deg.0 + 0.3 };
        let rgb = Hsv::new(deg, 1., 1.).to_rgb();
        canvas.set_draw_color(Color::RGB(rgb.r, rgb.g, rgb.b));
        canvas.clear();
        canvas.present();

        // event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }

            // canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
