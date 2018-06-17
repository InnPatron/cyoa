use conrod;
use conrod::{Colorable, Widget, Sizeable, Positionable, Borderable, Labelable};
use conrod::backend::glium::glium;
use conrod::widget;
use conrod::color;
use conrod::event::Input;
use conrod::input::{Button, Key};

use script_lib;
use game::*;

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
                       mut instance: GameInstance,
                       ) {
    let ids = GameIds::new(ui.widget_id_generator());
    let idle_ids = IdleScreenIds::new(ui.widget_id_generator());

    events_loop.run_forever(|event| {
        match event.clone() {
            glium::glutin::Event::WindowEvent { event, .. } => match event {
                glium::glutin::WindowEvent::Closed => return glium::glutin::ControlFlow::Break,
                _ => (),
            },
            _ => (),
        }

        // Use the `winit` backend feature to convert the winit event to a conrod one.
        let input = match conrod::backend::winit::convert_event(event, &display) {
            None => return glium::glutin::ControlFlow::Continue,
            Some(input) => input,
        };

        if let Input::Press(ref button) = input {
            if let Button::Keyboard(ref key) = *button {
                match *key {
                    Key::Escape => return glium::glutin::ControlFlow::Break,
                    _ => (),
                }
            }
        }
        // Handle the input with the `Ui`.
        ui.handle_event(input);
        {
            let game_state = instance.context().state();

            if game_state == script_lib::STATE_RUN {

                draw_game_screen(ui.set_widgets(), &ids, &mut instance);

            } else if game_state == script_lib::STATE_IMAGE {

                if draw_image_screen(ui.set_widgets(), &idle_ids, &instance) {
                    let context = instance.context_mut();
                    context.set_state(script_lib::STATE_RUN);
                    //context.vm.borrow_mut().eval_str("state 0;").unwrap();
                }

            } else if game_state == script_lib::STATE_END {
                let context = instance.context_mut();
                context.set_state(script_lib::STATE_RUN);
                return glium::glutin::ControlFlow::Break;
            } else {

                eprintln!("Invalid state {}", game_state);
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

fn draw_image_screen(ref mut ui: conrod::UiCell, ids: &IdleScreenIds, instance: &GameInstance) -> bool {
    widget::Canvas::new()
        .color(color::BLACK)
        .set(ids.canvas, ui);

    let context = instance.context();

    // TODO: Handle empty background string?
    let image_name = context.background();
    let image = instance.get_image(&image_name)
        .expect(&format!("Could not find image {}", image_name));

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

fn draw_game_screen(ref mut ui: conrod::UiCell, ids: &GameIds, instance: &mut GameInstance) {

    let text_row = widget::Canvas::new().color(color::BLACK)
        .scroll_kids_vertically();
    let option_row = widget::Canvas::new().color(color::BLACK);
    widget::Canvas::new().flow_down(&[
        (ids.text_row, text_row),
        (ids.option_row, option_row)
    ])
        .set(ids.canvas, ui);

    {
        let context = instance.context();
        let font = instance
            .get_font(&context.font())
            .expect("Unknown font").clone();

        let font_size = 24;

        let text: String = context.display();

        widget::Text::new(&*text)
            .font_id(font.id)
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
    }

    {
        let options = instance.context().choices();
        let len = options.len();
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
                    let display = options.get(item.i).unwrap().display();

                    let (color, label_color) = (color::GREY, color::BLACK);
                    let button = widget::Button::new()
                        .border(1.0)
                        .color(color)
                        .label(&display)
                        .left_justify_label()
                        .label_font_size(font_size)
                        .label_color(label_color);
                    item.set(button, ui);
                }

                Event::Selection(index) => {
                    let choice = options.get(index).unwrap();
                    instance.execute_choice(choice);
                }
                _ => (),
            }
        }

        if let Some(s) = scrollbar { s.set(ui); }
    }
}
