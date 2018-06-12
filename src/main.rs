extern crate smpl;
#[macro_use]
extern crate conrod;
extern crate find_folder;
extern crate image;
extern crate toml;
#[macro_use]
extern crate serde_derive;

mod library;
mod game;
mod title_screen;
mod game_screen;

use conrod::backend::glium::glium;

use game::Context;

use std::fs::File;
use std::io::Read;

fn main() {
    const WIDTH: u32 = 1080;
    const HEIGHT: u32 = 720;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    loop {
        let handle = match title_screen::handle_title_screen(&mut events_loop, &mut ui, display.clone(), &mut renderer, &image_map) {
            Some(handle) => handle,
            None => return,
        };

        let context = Context::new(&handle, &mut ui, &display, &mut image_map);
        let main = {
            let mut buffer = String::new();
            let mut handle = context.assets.scripts.get("main")
                .expect("Could not find main.popstcl in scripts folder");
            handle.read_to_string(&mut buffer).unwrap();
            buffer
        };

        // TODO: Eval main

        game_screen::handle_game_screen(&mut events_loop, &mut ui, display.clone(), &mut renderer, &image_map, context);
    }
}
