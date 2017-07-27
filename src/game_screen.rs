use find_folder::Search;
use conrod;
use conrod::{Scalar, Colorable, Widget, Sizeable, Positionable, Borderable, Labelable};
use conrod::backend::glium::glium;
use conrod::text::font::Id;
use conrod::widget;
use conrod::color;

use library;
use library::StoryHandle;
use story::StoryAssets;
use popstcl_core::*;
use popstcl_core::internal::Vm;

widget_ids! {
    struct GameIds { canvas, option_row, text_row, background_img, text, option_list }
}

pub fn handle_title_screen(events_loop: &mut glium::glutin::EventsLoop, 
                       ui: &mut conrod::Ui, 
                       display: glium::Display,
                       renderer: &mut conrod::backend::glium::Renderer,
                       image_map: &conrod::image::Map<glium::texture::Texture2d>
                       ) {
    let game_ids = GameIds::new(ui.widget_id_generator());

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

fn draw_game_screen(ref mut ui: conrod::UiCell, ids: &GameIds, assets: &StoryAssets, vm: &mut Vm) {
    
}
