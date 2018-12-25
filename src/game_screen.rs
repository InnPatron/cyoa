use std::io::Write;
use std::io;

use failure::Error;

use crate::game::GameInstance;
use crate::script_lib;

use crate::display;
use crate::input::*;

pub fn game_screen(mut game_instance: GameInstance) -> Result<(), Error> {
    loop {
        let game_state = game_instance.context().state();

        match game_state {
            script_lib::STATE_RUN => game_iteration(&mut game_instance)?,

            script_lib::STATE_END => break,

            _ => panic!("Unknown state: {}", game_state),
        }
    };

    Ok(())
}

fn game_iteration(game_instance: &mut GameInstance) -> Result<(), Error> {
    let choices = game_instance.context().choices();

    display::list(choices
                  .iter()
                  .map(|c| c.display())
                  .enumerate());

    prompt();
    let choice = loop {
        match number() {

            InputResult::Quit => {
                println!("Exiting...\n");
            }

            InputResult::Invalid(String) => {
                println!("Unknown choice.");
            },

            InputResult::Item(i) => {
                if i < 0 || i as usize >= choices.len() {
                    println!("Unknown choice.");
                } else {
                    break &choices[i as usize];
                }
            }
        }
    };

    game_instance.execute_choice(choice)
}

fn prompt() {
    print!("> ");
    io::stdout().flush().expect("IO ERROR");
}
