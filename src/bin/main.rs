use color::{Deg, Hsv, ToRgb};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use std::mem;
use std::os::raw::c_void;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};

mod shader;
mod vertex;

use shader::Shader;
use vertex::Vertex;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 3;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    // init window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    {
        // set OpenGL profile and version to GLAttr
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core); // Core
        gl_attr.set_context_version(3, 1); // Version 3.1
        let (major, minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version={}.{}", major, minor);
    }
    // gl_attr is released here

    let width = 640;
    let height = 480;
    let window = video_subsystem
        .window("SDL", width, height)
        .build()
        .unwrap();

    // create OpenGL context
    let _gl_context = window.gl_create_context().unwrap();
    // load OpenGL APIs
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    // let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");

    // set buffer
    #[rustfmt::skip]
    let buffer_array: [f32; BUF_LEN] = [
        -1.0, -1.0, 0.0,
        1.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
    ];

    // let vertext = Vertex::new();

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
