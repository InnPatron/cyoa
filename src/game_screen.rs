use failure::Error;

use crate::game::GameInstance;
use crate::script_lib;

use crate::display;
use crate::input::*;

enum Decision {
    Continue,
    Quit,
}

pub fn game_screen(mut game_instance: GameInstance) -> Result<(), Error> {
    loop {
        let game_state = game_instance.context().state();

        match game_state {
            script_lib::STATE_RUN => {
                match game_iteration(&mut game_instance)? {
                    Decision::Continue => (),
                    
                    Decision::Quit => return Ok(()),
                }
            }

            script_lib::STATE_END => break,

            _ => panic!("Unknown state: {}", game_state),
        }
    };

    Ok(())
}

fn game_iteration(game_instance: &mut GameInstance) -> Result<Decision, Error> {
    let choices = game_instance.context().choices();

    display::list(choices
                  .iter()
                  .map(|c| c.display())
                  .enumerate());

    let choice = loop {

        display::prompt(true, false);
        match number() {

            InputResult::Quit => {
                println!("Exiting...\n");
                return Ok(Decision::Quit);
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

    game_instance.execute_choice(choice)?;

    return Ok(Decision::Continue);
}
