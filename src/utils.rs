use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;

pub fn read_input(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn read_booklist() {
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

pub fn get_booklist() -> String {
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
            return s;
        }
    }
}

pub fn get_return_list(name: String) -> String {
    let mut file = match File::open(format!("members/{}.txt", name.trim())) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", why),
        Ok(_) => {
            return s;
        }
    }
}
