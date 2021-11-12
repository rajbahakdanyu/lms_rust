mod borrow_book;
mod display_booklist;
mod return_book;
mod utils;

fn main() {
    println!("Welcome to the Library of Alexandria!");
    println!("-------------------------------------");

    loop {
        let mut operation = String::new();

        println!(
            "Select an action by pressing \n1. For Borrowing a Book \n2. For Returning a Book \n3. To Exit"
        );
        utils::read_input(&mut operation);

        let operation: char = operation.trim().chars().next().unwrap_or('8');

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
