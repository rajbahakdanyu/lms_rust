mod borrow_book;
mod display_booklist;
mod return_book;

use std::io::{stdin, stdout, Write};

fn read_input(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("Welcome to the Library of Alexandria!");
    println!("-------------------------------------");

    loop {
        let mut operation = String::new();

        println!(
            "Select an action by pressing \n1. For Renting Book \n2. For Returning \n3. To Exit"
        );
        read_input(&mut operation);

        let operation: char = operation.trim().chars().next().unwrap();

        let operater = String::from("123");

        if !operater.contains(operation) {
            println!("Please enter a valid option");
            continue;
        }
        match operation {
            '1' => borrow_book::borrow_book(),
            '2' => return_book::return_book(),
            '3' => break,
            _ => panic!("error in operator"),
        };
    }
    println!("Good Bye");
}
