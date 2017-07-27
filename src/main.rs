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

fn main() {
    feature::main();
}

mod feature {
    use std::collections::HashSet;
    use find_folder::Search;
    use conrod;
    use conrod::{Scalar, Colorable, Widget, Sizeable, Positionable, Borderable, Labelable};
    use conrod::backend::glium::glium;
    use conrod::text::font::Id;
    use conrod::widget;
    use conrod::color;

    use super::library;
    use super::library::StoryHandle;
    use story::StoryAssets;

    use popstcl_core::*;
    use popstcl_core::internal::Vm;

    widget_ids! {
        struct TitleIds { canvas, option_row, title_row, story_list, scrollbar, title, option_right, option_left, game_start}
    }

    widget_ids! {
        struct GameIds { canvas, option_row, text_row, background_img, text, option_list }
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

        let game_ids = TitleIds::new(ui.widget_id_generator());

        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        let (vm_out, mut vm) = {
            use super::commands::Display;
            let mut vm = basic_vm();
            let display = RcValue::new(0.0.into_value());
            vm.insert_value("display", Value::Cmd(Box::new(Display(display.clone()))));
            (display, vm)
        };

        handle_title_screen(&mut events_loop, &mut ui, display.clone(), &mut renderer, &image_map);
        
    }

    fn handle_title_screen(events_loop: &mut glium::glutin::EventsLoop, 
                           ui: &mut conrod::Ui, 
                           display: glium::Display,
                           renderer: &mut conrod::backend::glium::Renderer,
                           image_map: &conrod::image::Map<glium::texture::Texture2d>
                           ) {
        let title_ids = TitleIds::new(ui.widget_id_generator());
        let assets = Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let noto_sans = assets.join("fonts/NotoSans");
        let mut handles = library::scan_library();

        let fonts = Fonts {
            regular: ui.fonts.insert_from_file(noto_sans.join("NotoSans-Regular.ttf")).unwrap(),
            italic: ui.fonts.insert_from_file(noto_sans.join("NotoSans-Italic.ttf")).unwrap(),
            bold: ui.fonts.insert_from_file(noto_sans.join("NotoSans-Bold.ttf")).unwrap(),
        };
        let mut selection = None;

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

            {
                if draw_title_screen(ui.set_widgets(), &title_ids, &fonts, &handles, &mut selection) {
                }
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

    fn draw_title_screen(ref mut ui: conrod::UiCell, ids: &TitleIds, fonts: &Fonts, handles: &[StoryHandle], selection: &mut Option<usize>) -> bool {

        let option_inner = &[
            (ids.option_left, widget::Canvas::new()
             .color(color::GREY)
             .length_weight(1.8)),
            (ids.option_right, widget::Canvas::new().color(color::GREY))
            ];
        let option_row = widget::Canvas::new()
            .flow_right(option_inner);
        widget::Canvas::new().flow_down(&[
            (ids.title_row, widget::Canvas::new().color(color::BLACK)),
            (ids.option_row, option_row)
        ])
            .set(ids.canvas, ui);


        const TITLE_PAD: Scalar = 15.0;
        widget::Text::new("CYOA")
            .font_id(fonts.italic)
            .middle_of(ids.title_row)
            .center_justify()
            .color(color::WHITE)
            .font_size(120)
            .set(ids.title, ui);
        
        let submit = widget::Button::new()
            .label("PLAY")
            .center_justify_label()
            .padded_wh_of(ids.option_right, 10.)
            .middle_of(ids.option_right)
            .set(ids.game_start, ui)
            .was_clicked();

        const LIST_WPAD: Scalar = 20.0;
        let items = handles.len();
        let item_h = 50.0;
        let font_size = item_h as conrod::FontSize / 2;
        let (mut events, scrollbar) = widget::ListSelect::single(items)
            .flow_down()
            .item_size(item_h)
            .scrollbar_next_to()
            .padded_w_of(ids.option_left, LIST_WPAD)
            .top_left_with_margins_on(ids.option_left, 20.0, 20.0)
            .set(ids.story_list, ui);
        

        while let Some(event) = events.next(ui, |i| true) {
            use conrod::widget::list_select::Event;
            match event {
                Event::Item(item) => {
                    let label = &handles[item.i].metadata.name;
                    let (color, label_color) = { 
                        let not_selected = (color::LIGHT_GREY, color::BLACK);
                        let selected = (color::LIGHT_GREY, color::RED);
                        match *selection {
                            Some(i) => {
                                if i == item.i {
                                    selected
                                } else {
                                    not_selected
                                }
                            }

                            None => not_selected
                        }  
                    };
                    let button = widget::Button::new()
                        .border(0.0)
                        .color(color)
                        .label(label)
                        .left_justify_label()
                        .label_font_size(font_size)
                        .label_color(label_color);
                    item.set(button, ui);
                }

                Event::Selection(index) => {
                    *selection = Some(index);
                }

                _ => (),
            }
        }

        if let Some(s) = scrollbar { s.set(ui); }
        submit
    }

    fn draw_game_screen(ref mut ui: conrod::UiCell, ids: &GameIds, assets: &StoryAssets, vm: &mut Vm) {
        
    }
}


