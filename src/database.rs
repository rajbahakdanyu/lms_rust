use std::{fs::OpenOptions, io::Write};

use crate::utils::get_booklist;

pub fn database(return_type: String, book: Vec<&str>) {
    let list = get_booklist();
    let booklist: Vec<&str> = list.trim().split("\r\n").collect();

    if return_type == "b" {
        let mut file = OpenOptions::new()
            .write(true)
            .open(format!("booklist.txt"))
            .unwrap();

        for elem in booklist {
            let temp: Vec<&str> = elem.trim().split(",").collect();
            if temp[0] == book[0] {
                file.write_all(
                    format!(
                        "{},{},{},{},{}\n",
                        temp[0],
                        temp[1],
                        temp[2],
                        &(book[3].to_string().parse::<i32>().unwrap() - 1).to_string(),
                        temp[4],
                    )
                    .as_bytes(),
                )
                .unwrap();
            } else {
                file.write_all(
                    format!(
                        "{},{},{},{},{}\n",
                        temp[0], temp[1], temp[2], temp[3], temp[4],
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
        }
        println!("{} borrowed successfully\n", book[1]);
    } else if return_type == "r" {
        println!("user is returning\n")
    }
}
