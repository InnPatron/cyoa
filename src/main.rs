extern crate smpl;
extern crate find_folder;
extern crate toml;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate irmatch;
#[macro_use]
extern crate failure;

mod input;
mod script_lib;
mod library;
mod game;
mod assets;
mod display;

mod game_screen;
mod title_screen;

use failure::Error;

use crate::game::GameInstance;

fn main() {
    let library = library::scan_library();
    let story = title_screen::title_screen(&library);

    match run(story) {
        Ok(_) => (),

        Err(e) => {
            println!("{}", e);
            return;
        }
    }
}

fn run(story: Option<&library::StoryHandle>) -> Result<(), Error> {
    match story {
        Some(story) => {
            let game_instance = game::GameInstance::new(story)?;

            game_screen::game_screen(game_instance)?;

            Ok(())
        }

        None => {
            Ok(())
        }
    }
}
