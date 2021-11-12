use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;

fn read_input(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn read_booklist() {
    let path = Path::new("booklist.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let temp: Vec<&str> = s.split('\n').collect();
            println!("Book Id\tBook Name\tAuthor\t\tQuantity\tPrice");
            for elem in temp {
                let temp2: Vec<&str> = elem.split(',').collect();
                println!(
                    "{}\t{}\t{}\t{}\t\t{}",
                    temp2[0], temp2[1], temp2[2], temp2[3], temp2[4]
                );
            }
            println!("");
        }
    }
}

fn main() {
    println!("Welcome to the Library of Alexandria!");
    println!("-------------------------------------");
    read_booklist();

    loop {
        let mut operation = String::new();

        println!(
            "Select an action by pressing \n1. For Renting Book \n2. For Returning \n3. To Exit."
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
