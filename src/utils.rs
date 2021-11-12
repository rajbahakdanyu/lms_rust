use std::io::{stdin, stdout, Write};

pub fn read_input(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}
