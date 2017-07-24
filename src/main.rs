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

fn main() {
    feature::main();
}

mod feature {
    use find_folder::Search;
    use conrod;
    use conrod::backend::glium::glium;
    use conrod::text::font::Id;

    widget_ids! {
        struct Ids { canvas, text, scrollbar }
    }

    struct Fonts {
        regular: Id,
        italic: Id,
        bold: Id
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

        let ids = Ids::new(ui.widget_id_generator());

        let assets = Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let noto_sans = assets.join("fonts/NotoSans");

        let fonts = Fonts {
            regular: ui.fonts.insert_from_file(noto_sans.join("NotoSans-Regular.ttf")).unwrap(),
            italic: ui.fonts.insert_from_file(noto_sans.join("NotoSans-Italic.ttf")).unwrap(),
            bold: ui.fonts.insert_from_file(noto_sans.join("NotoSans-Bold.ttf")).unwrap(),
        };

        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        events_loop.run_forever(|event| {
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => return glium::glutin::ControlFlow::Break,
                    _ => (),
                },
                _ => (),
            }

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => return glium::glutin::ControlFlow::Continue,
                Some(input) => input,
            };

            // Handle the input with the `Ui`.
            ui.handle_event(input);

            let DEMO = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbllllllllllllllllllllllllllllllllllllllllllllllllllaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbllllllllllllllllllllllllllllllllllllllllllllllllllaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbllllllllllllllllllllllllllllllllllllllllllllllllllaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
            {
                let mut ui = ui.set_widgets();
                use conrod::{self, color, widget, Colorable, Positionable, Sizeable, Widget };
                use conrod::backend::glium::glium::{self, Surface};
                widget::Canvas::new()
                    .scroll_kids_vertically()
                    .color(color::DARK_CHARCOAL)
                    .set(ids.canvas, &mut ui);

                widget::Text::new(DEMO)
                    .font_id(fonts.regular)
                    .color(color::WHITE)
                    .left_justify()
                    .line_spacing(10.0)
                    .scroll_kids_vertically()
                    .padded_w_of(ids.canvas, 20.0)
                    .mid_top_of(ids.canvas)
                    .set(ids.text, &mut ui);

                widget::Scrollbar::y_axis(ids.canvas).auto_hide(false).set(ids.scrollbar, &mut ui);
            }

            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                //target.clear_color(0.0, 0.0, 0.0, 1.0);
                glium::Surface::clear_color(&mut target, 0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }

            glium::glutin::ControlFlow::Continue
        });
    }
}
