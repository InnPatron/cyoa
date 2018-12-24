use std::io;

pub enum InputResult<I> {
    Item(I),
    Invalid(String),
    Quit,
}

pub fn number() -> InputResult<i64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input).unwrap();

    if input == "q" {
        return InputResult::Quit;
    }

    match input.parse::<i64>() {
        Ok(i) => InputResult::Item(i),
        Err(_) => InputResult::Invalid(input)
    }
}
