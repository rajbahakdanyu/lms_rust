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

                    for elem in &temp {
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
                    let mut book_name = String::new();
                    let mut check_book = true;
                    let mut return_list = Vec::new();

                    println!("Enter book name:");
                    read_input(&mut book_name);

                    for elem in temp {
                        let temp2: Vec<&str> = elem.split(',').collect();

                        if elem.len() > 0 {
                            if temp2[0].to_lowercase() == book_name.trim().to_lowercase()
                                && temp2[4] == "not returned"
                            {
                                check_book = false;
                                return_list = temp2;
                            }
                        }
                    }

                    if check_book {
                        println!("{} has not borrowed {}", borrower_name.trim(), book_name);
                    } else {
                        book_return(borrower_name.trim(), book_name, return_list);
                    }
                }
            }
        } else {
            println!("{} has not borrowed any books\n", borrower_name.trim());
        }
    } else {
        println!("Please enter a name");
        return_book();
    }
}

fn book_return(name: &str, book: String, return_list: Vec<&str>) {
    let current = chrono::Local::now().format("%d-%m-%y %H:%M").to_string();
    let current_date = chrono::NaiveDateTime::parse_from_str(&current, "%d-%m-%y %H:%M").unwrap();
    let deadline = chrono::NaiveDateTime::parse_from_str(return_list[3], "%d-%m-%y %H:%M").unwrap();

    if current_date > deadline {
        println!("Past deadline");
    } else {
        println!("Within deadline");
    }
}
