use std::fs;

use crate::utils::read_input;

pub fn return_book() {
    let mut borrower_name = String::new();
    let mut check_name = false;

    println!("Enter borrower name: ");
    read_input(&mut borrower_name);

    if borrower_name.trim() != "" {
        match fs::read_to_string("namelist.txt") {
            Err(why) => panic!("Error: {}", why),
            Ok(s) => {
                let temp: Vec<&str> = s.split("\r\n").collect();
                if temp.iter().any(|&i| i == borrower_name.trim()) {
                    check_name = true;
                }
            }
        }

        if check_name {
            match fs::read_to_string(format!("members/{}.txt", borrower_name.trim())) {
                Err(why) => panic!("Error: {}", why),
                Ok(s) => {
                    let temp: Vec<&str> = s.split("\r\n").collect();
                    println!("\nBook name\t\tPrice($)\tBorrow date\t\tDeadline\t\tStatus");

                    for elem in temp {
                        if elem.len() > 0 {
                            let temp2: Vec<&str> = elem.split(',').collect();

                            if temp2[4] == "not returned" {
                                println!(
                                    "{}\t\t{}\t\t{}\t\t{}\t\t{}",
                                    temp2[0], temp2[1], temp2[2], temp2[3], temp2[4]
                                );
                            }
                        }
                    }
                    println!("");
                }
            }
        } else {
            println!("{} has not borrowed any books", borrower_name.trim());
        }
    } else {
        println!("Please enter a name");
        return_book();
    }
}
