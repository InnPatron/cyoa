#[macro_use]
extern crate popstcl_core;
#[macro_use]
extern crate conrod;
extern crate find_folder;
extern crate image;
extern crate toml;
#[macro_use]
extern crate serde_derive;

mod library;
mod game;
mod commands;
mod title_screen;
mod game_screen;

use find_folder::Search;
use conrod::{Scalar, Colorable, Widget, Sizeable, Positionable, Borderable, Labelable};
use conrod::backend::glium::glium;
use conrod::text::font::Id;
use conrod::widget;
use conrod::color;

use library::StoryHandle;
use game::{Context, StoryAssets };

use popstcl_core::*;
use popstcl_core::internal::Vm;
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

    let handle = title_screen::handle_title_screen(&mut events_loop, &mut ui, display.clone(), &mut renderer, &image_map);
    let main = {
        let mut buffer = String::new();
        let mut path = handle.root.clone();
        path.push(&handle.metadata.main);
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut buffer);
        buffer
    };

    let context = Context::new(&handle, &mut ui, &display, &mut image_map);
    context.vm.borrow_mut().eval_str(&main).unwrap();

    game_screen::handle_game_screen(&mut events_loop, &mut ui, display.clone(), &mut renderer, &image_map, context);
}
