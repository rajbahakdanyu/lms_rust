use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;

fn read_input(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn read_booklist() {
    // Create a path to the desired file
    let path = Path::new("booklist.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("Current Selection:\n{}", s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn main() {
    println!("Welcome to the Library of Alexandria!");
    println!("-------------------------------------");

    loop {
        read_booklist();
        let mut operation = String::new();

        println!("Press \n1. For Renting Book \n2. For Returning \n3. To Exit.");
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

        println!("You chose {}", result);
    }
    println!("Good Bye");
}
