use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("Welcome to the Library of Alexandria!");
    println!("-------------------------------------");

    loop {
        let mut operation = String::new();

        println!("Press \n1. For Renting Book \n2. For Returning \n3. To Exit.");
        read(&mut operation);

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

        println!("You chose {}", result);
    }
    println!("Good Bye");
}
