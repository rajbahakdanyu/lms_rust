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
            let list = get_booklist();
            let temp: Vec<&str> = s.split("\n").collect();
            let booklist: Vec<&str> = list.trim().split("\r\n").collect();
            let re = Regex::new(r"(^[0-9]*$)").unwrap();
            let mut book_id = String::new();
            println!("");
            read_booklist();
            println!("Enter book id: ");
            read_input(&mut book_id);

            if re.is_match(book_id.trim()) {
                let mut check_book = false;
                let mut check_available = true;
                let mut book_name = String::new();

                // Check if book exists in book list
                for book in booklist {
                    let each_book: Vec<&str> = book.split(",").collect();

                    if each_book[0] == book_id.trim() {
                        check_book = true;
                        book_name = each_book[1].to_string();
                    }
                }

                if check_book {
                    // Check if user has already borrowed this book
                    for elem in temp {
                        let temp2: Vec<&str> = elem.split(",").collect();

                        if temp2[0] == book_name.trim() && temp2[4] == "not returned" {
                            check_available = false;
                        }
                    }

                    if check_available {
                        println!("User can borrow this book");
                    } else {
                        println!("User cannot borrow this book");
                    }
                } else {
                    println!("Book does not exit");
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
