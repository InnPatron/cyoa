use std::iter::{Enumerate, ExactSizeIterator};
use std::fmt::Display;

use std::io::Write;
use std::io;

pub fn list<I, T>(iterator: Enumerate<I>) -> Option<usize> 
    where I: ExactSizeIterator + Iterator<Item = T>, T: Display {

    for (index, display) in iterator {
        println!("{}: {}", index, display);
    }

    None
}


pub fn prompt(nl_before: bool, nl_after: bool) {
    if nl_before {
        print!("\n");
    }

    print!("> ");

    if nl_after {
        print!("\n");
    }

    io::stdout().flush().expect("IO ERROR");
}
