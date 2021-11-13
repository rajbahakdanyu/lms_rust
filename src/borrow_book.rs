use crate::display_booklist::read_booklist;
use crate::utils::read_input;

use std::fs;

pub fn borrow_book() {
    println!("");
    read_booklist();
    let mut borrower_name = String::new();

    println!("Enter borrower name: ");
    read_input(&mut borrower_name);

    if borrower_name.trim() != "" {
        match fs::read_to_string("namelist.txt") {
            Err(why) => panic!("Error: {}", why),
            Ok(s) => {
                let temp: Vec<&str> = s.split("\r\n").collect();
                if temp.iter().any(|&i| i == borrower_name.trim()) {
                    old_borrower()
                } else {
                    new_borrower()
                }
            }
        }
    } else {
        println!("Please enter a name");
        borrow_book();
    }
}

fn old_borrower() {
    println!("Old borrower");
}

fn new_borrower() {
    println!("New borrower");
}
