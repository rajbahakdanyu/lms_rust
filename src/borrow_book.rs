use crate::display_booklist::read_booklist;
use crate::utils::read_input;

use std::fs;

pub fn borrow_book() {
    println!("");
    read_booklist();
    let mut borrower_name = String::new();

    println!("Enter borrower name: ");
    read_input(&mut borrower_name);

    match fs::read_to_string("namelist.txt") {
        Err(why) => panic!("Error: {}", why),
        Ok(s) => {
            let temp: Vec<&str> = s.split("\n\t").collect();

            if temp.iter().any(|&i| i == borrower_name) {
                println!("Yes");
            } else {
                println!("Oh no")
            }
        }
    }
}
