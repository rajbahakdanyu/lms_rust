use crate::utils::{get_booklist, read_booklist, read_input};

use regex::Regex;
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
            let re = Regex::new(r"(^[0-9]*$)").unwrap();
            let mut book_id = String::new();
            println!("");
            read_booklist();
            println!("Enter book id: ");
            read_input(&mut book_id);

            let s = get_booklist();

            let booklist: Vec<&str> = s.trim().split("\r\n").collect();
            println!("{:?}", booklist);

            if re.is_match(book_id.trim()) {
                let mut check_book = false;
                for elem in temp {
                    let temp2: Vec<&str> = elem.split(",").collect();
                    if temp2.iter().any(|&i| i == book_id.trim()) {
                        check_book = true;
                    }
                }
                if check_book {
                    println!("{} has already borrowed this book", borrower_name);
                }
            } else {
                println!("Book Id should be a number\n");
            }
        }
    }
}

fn new_borrower() {
    println!("New borrower");
}
