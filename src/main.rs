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
mod story;
mod commands;
mod title_screen;

fn main() {
    feature::main();
}

mod feature {
    use find_folder::Search;
    use conrod;
    use conrod::{Scalar, Colorable, Widget, Sizeable, Positionable, Borderable, Labelable};
    use conrod::backend::glium::glium;
    use conrod::text::font::Id;
    use conrod::widget;
    use conrod::color;

    use super::library;
    use super::library::StoryHandle;
    use super::title_screen;
    use story::StoryAssets;

    use popstcl_core::*;
    use popstcl_core::internal::Vm;

    widget_ids! {
        struct GameIds { canvas, option_row, text_row, background_img, text, option_list }
    }

    pub fn main() {
        const WIDTH: u32 = 1080;
        const HEIGHT: u32 = 720;

        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_dimensions(WIDTH, HEIGHT);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        let game_ids = GameIds::new(ui.widget_id_generator());

        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        let (vm_out, mut vm) = {
            use super::commands::Display;
            let mut vm = basic_vm();
            let display = RcValue::new(0.0.into_value());
            vm.insert_value("display", Value::Cmd(Box::new(Display(display.clone()))));
            (display, vm)
        };

        let handle = title_screen::handle_title_screen(&mut events_loop, &mut ui, display.clone(), &mut renderer, &image_map);
        
    }

    fn draw_game_screen(ref mut ui: conrod::UiCell, ids: &GameIds, assets: &StoryAssets, vm: &mut Vm) {
        
    }
}


