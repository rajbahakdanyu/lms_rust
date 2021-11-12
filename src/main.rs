mod display_booklist;

use std::io::{stdin, stdout, Write};

fn read_input(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("Welcome to the Library of Alexandria!");
    println!("-------------------------------------");
    display_booklist::read_booklist();

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

        let result = match operation {
            '1' => '1',
            '2' => '2',
            '3' => break,
            _ => panic!("error in operator"),
        };

        println!("You chose {}\n", result);
    }
    println!("Good Bye");
}
