use crate::display_booklist::read_booklist;
use crate::utils::read_input;

use std::fs;

pub fn borrow_book() {
    let mut borrower_name = String::new();

    println!("Enter borrower name: ");
    read_input(&mut borrower_name);

    if borrower_name.trim() != "" {
        match fs::read_to_string("namelist.txt") {
            Err(why) => panic!("Error: {}", why),
            Ok(s) => {
                let temp: Vec<&str> = s.split("\r\n").collect();
                if temp.iter().any(|&i| i == borrower_name.trim()) {
                    old_borrower(borrower_name.trim())
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

fn old_borrower(borrower_name: &str) {
    match fs::read_to_string(format!("members/{}.txt", borrower_name)) {
        Err(why) => panic!("Error: {}", why),
        Ok(s) => {
            let temp: Vec<&str> = s.split("\n").collect();

            let mut book_id = String::new();
            println!("");
            read_booklist();
            println!("Enter book id: ");
            read_input(&mut book_id);

            for elem in temp {
                let temp2: Vec<&str> = elem.split(",").collect();

                println!("{:?}", temp2);
            }
        }
    }
}

fn new_borrower() {
    println!("New borrower");
}
