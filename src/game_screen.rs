use conrod;
use conrod::{Colorable, Widget, Sizeable, Positionable, Borderable, Labelable};
use conrod::backend::glium::glium;
use conrod::widget;
use conrod::color;

use game::Context;
use popstcl_core::*;

widget_ids! {
    struct GameIds { canvas, background, text_row, text_scroll, text, option_row, option_list }
}

widget_ids! {
    struct IdleScreenIds { canvas, background }
}

pub fn handle_game_screen(events_loop: &mut glium::glutin::EventsLoop, 
                       ui: &mut conrod::Ui, 
                       display: glium::Display,
                       renderer: &mut conrod::backend::glium::Renderer,
                       image_map: &conrod::image::Map<glium::texture::Texture2d>,
                       context: Context
                       ) {
    let ids = GameIds::new(ui.widget_id_generator());
    let idle_ids = IdleScreenIds::new(ui.widget_id_generator());

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
            let game_state: f64 = *context.game_state.borrow().try_into_number().expect("Game state should only be a number");
            if game_state == 0.0 {
                draw_game_screen(ui.set_widgets(), &ids, &context);
            } else if game_state == 1.0 {
                if draw_image_screen(ui.set_widgets(), &idle_ids, &context) {
                    context.vm.borrow_mut().eval_str("state 0;").unwrap();
                }
            } else if game_state == 2.0 {
                context.vm.borrow_mut().eval_str("state 0").unwrap();
                return glium::glutin::ControlFlow::Break;
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

fn draw_image_screen(ref mut ui: conrod::UiCell, ids: &IdleScreenIds, context: &Context) -> bool {
    widget::Canvas::new()
        .color(color::BLACK)
        .set(ids.canvas, ui);

    let image_name = context.vm.borrow().get("background")
        .expect("'background' variable expected to display background");
    let image_name = image_name.borrow()
        .try_into_string()
        .expect("'background' variable should only be a String");
    let image = context.assets.images.get(&**image_name)
        .expect(&format!("{} is an invalid image (PNG's only, name without file extension)", &**image_name));
    if widget::Button::image(image.id)
        .border(0.0)
        .wh_of(ids.canvas)
        //.w_h(image.w as conrod::Scalar, image.h as conrod::Scalar)
        .middle()
        .set(ids.background, ui)
        .was_clicked() {
            true
    } else {
            false
    }
}

fn draw_game_screen(ref mut ui: conrod::UiCell, ids: &GameIds, context: &Context) {

    let text_row = widget::Canvas::new().color(color::BLACK)
        .scroll_kids_vertically();
    let option_row = widget::Canvas::new().color(color::BLACK);
    widget::Canvas::new().flow_down(&[
        (ids.text_row, text_row),
        (ids.option_row, option_row)
    ])
        .set(ids.canvas, ui);

    let font = context.assets.fonts.get(
        &context.vm.borrow().get_value("dispfont")
            .map(|v| (**v.try_into_string().expect("display font should only be a string")).to_string())
            .unwrap()//.expect("Require variable 'dispfont' to determine font of the display")
    ).expect("Unknown font").clone();

    let font_size = 24;

    let text: &str = &**context.vm_out
        .inner_clone()
        .try_into_string()
        .expect("vm_out should be a string");

    widget::Text::new(text)
        .font_id(font)
        .top_left_with_margin_on(ids.text_row, 5.)
        .padded_w_of(ids.text_row, 10.)
        .left_justify()
        .color(color::WHITE)
        .font_size(font_size)
        .set(ids.text, ui);
    widget::Scrollbar::y_axis(ids.text_row)
        .auto_hide(false)
        .thickness(14.)
        .color(color::WHITE)
        .set(ids.text_scroll, ui);

    {
        let options = context.vm.borrow()
            .get_value("options")
            .expect("Missing 'options' (List) to display options")
            .try_into_list()
            .expect("'options' field should be a List");

        let len = options.inner().len();
        let item_h = 50.0;
        let font_size = item_h as conrod::FontSize / 2;
        let (mut events, scrollbar) = widget::ListSelect::single(len)
            .flow_down()
            .item_size(item_h)
            .scrollbar_next_to()
            .padded_w_of(ids.option_row, 15.0)
            .top_left_with_margin_on(ids.option_row, 15.0)
            .set(ids.option_list, ui);

        while let Some(event) = events.next(ui, |_| true) {
            use conrod::widget::list_select::Event;
            match event {
                Event::Item(item) => {
                    let value = options.inner().get(item.i).unwrap();
                    let value = value.borrow()
                        .try_into_object()
                        .expect("Options can only be objects");
                    let display = value.get("display")
                        .expect("Options should have a 'display' String");
                    let display: &str = &**display
                        .borrow()
                        .try_into_string()
                        .expect("'display' should only be a String");

                    let (color, label_color) = (color::GREY, color::BLACK);
                    let button = widget::Button::new()
                        .border(1.0)
                        .color(color)
                        .label(display)
                        .left_justify_label()
                        .label_font_size(font_size)
                        .label_color(label_color);
                    item.set(button, ui);
                }

                Event::Selection(index) => {
                    let value = options.inner().get(index).unwrap();
                    let value = value.borrow()
                        .try_into_object()
                        .expect("Options can only be objects");

                    let consequence = value.get("consequence")
                        .expect("Options should have a 'consequnce' String");
                    let consequence: &str = &**consequence
                        .borrow()
                        .try_into_string()
                        .expect("'consequence' should only be a String");
                    context.vm.borrow_mut().eval_str(consequence).unwrap();
                }
                _ => (),
            }
        }

        if let Some(s) = scrollbar { s.set(ui); }
    }
}
