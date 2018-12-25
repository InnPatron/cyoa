use crate::library::StoryHandle;
use crate::assets::*;
use crate::script_lib;

use crate::display;
use crate::input::{number, InputResult};

pub fn title_screen(handles: &[StoryHandle]) -> Option<&StoryHandle> {

    println!("CYOA\n");

    display::list(handles
                  .iter()
                  .map(|handle| &handle.metadata.name)
                  .enumerate());

    display::prompt(true, false);
    loop {
        match number() {
            InputResult::Quit => return None,
            InputResult::Invalid(String) => println!("Unknown index"),
            InputResult::Item(i) => {
                if i < 0 || i as usize >= handles.len() {
                    println!("Index out of range");
                } else {
                    return Some(&handles[i as usize])
                }
            }
        }
    }
}
