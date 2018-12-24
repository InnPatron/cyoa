use std::iter::{Enumerate, ExactSizeIterator};
use std::fmt::Display;

pub fn list<I, T>(mut iterator: Enumerate<I>) -> Option<usize> 
    where I: ExactSizeIterator + Iterator<Item = T>, T: Display {

    for (index, display) in iterator {
        println!("{}: {}", index, display);
    }

    None
}
